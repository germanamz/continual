//! The architecture check: which crates may depend on which.
//!
//! Every workspace member takes one role from `arch.toml`: a core (any crate under the
//! cores directory), a library crate (any crate under the libraries directory, when
//! `arch.toml` names one), a composition root, an adapter (any crate under the adapters
//! directory) or tooling. A library crate depends on no workspace crate, another
//! library crate included. A core depends on no workspace crate but library crates.
//! Both depend only on the external crates on their own allow-list. Adapters depend
//! only on cores, on library crates and on declared couplings. Only composition roots
//! depend on adapters, composition roots do not depend on each other, and product
//! crates do not depend on tooling.

pub mod sources;

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::finding::Finding;
use crate::git;

/// The rules in `arch.toml`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ArchConfig {
    /// Where the cores live and each core's external allow-list.
    pub cores: AllowListedCrates,
    /// Where the library crates live and each one's external allow-list. Absent while
    /// the workspace has no library crate.
    pub libraries: Option<AllowListedCrates>,
    /// The crates allowed to wire adapters, one per deployable.
    pub composition_roots: CrateList,
    /// Where adapters live and which of them may use each other.
    pub adapters: AdapterRules,
    /// Repository tooling crates.
    pub tooling: CrateList,
}

/// The `[cores]` table or the `[libraries]` table: a directory whose every crate takes
/// the table's role, and an external allow-list for each of those crates.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct AllowListedCrates {
    /// Directory, relative to the workspace root, holding every crate of the role.
    pub directory: PathBuf,
    /// The external crates each crate may depend on, by that crate's package name.
    #[serde(default)]
    pub allowed_dependencies: BTreeMap<String, Vec<AllowedDependency>>,
}

/// An external crate a core or a library crate may use, with the reason it is allowed.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllowedDependency {
    /// Package name.
    pub name: String,
    /// Why that crate may depend on it.
    pub reason: String,
}

/// A table holding a list of package names.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrateList {
    /// Package names.
    pub crates: Vec<String>,
}

/// The `[adapters]` table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterRules {
    /// Directory, relative to the workspace root, holding every adapter crate.
    pub directory: PathBuf,
    /// Adapter-to-adapter dependencies that are allowed.
    #[serde(default)]
    pub couplings: Vec<Coupling>,
}

/// One adapter allowed to depend on another.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Coupling {
    /// The depending adapter.
    pub from: String,
    /// The adapter it may depend on.
    pub to: String,
    /// Why the two cannot be separated.
    pub reason: String,
}

impl ArchConfig {
    /// Parses the text of `arch.toml`.
    ///
    /// # Errors
    ///
    /// Fails when the text is not TOML or does not match the schema above.
    pub fn parse(text: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(text)
    }
}

/// The workspace members and their declared dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceGraph {
    /// Every workspace member.
    pub packages: Vec<PackageNode>,
}

/// One workspace member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageNode {
    /// Package name.
    pub name: String,
    /// Directory of its manifest, relative to the workspace root.
    pub manifest_dir: PathBuf,
    /// Whether its manifest sets `[lints] workspace = true`.
    pub inherits_workspace_lints: bool,
    /// Its declared dependencies of every kind.
    pub dependencies: Vec<DependencyEdge>,
}

/// One declared dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyEdge {
    /// Package name of the dependency.
    pub name: String,
    /// Which section declares it.
    pub kind: DependencyKind,
    /// Whether the dependency is another workspace member.
    pub workspace_member: bool,
}

/// The manifest section a dependency is declared in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    /// `[dependencies]`.
    Normal,
    /// `[dev-dependencies]`.
    Dev,
    /// `[build-dependencies]`.
    Build,
}

/// Why the architecture check could not run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchError(pub String);

impl fmt::Display for ArchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ArchError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Core,
    Library,
    CompositionRoot,
    Adapter,
    Tooling,
}

/// The cores or the library crates: a role whose every crate has an allow-list, with
/// the words a finding uses for it.
struct AllowListed<'a> {
    role: Role,
    /// What a finding calls one crate of the role.
    noun: &'static str,
    /// The name of the role's table in `arch.toml`.
    table: &'static str,
    rules: &'a AllowListedCrates,
}

/// The check that reads one source file of a crate, which depends on the crate's role.
type SourceCheck = fn(&Path, &str) -> Result<Vec<Finding>, syn::Error>;

/// Runs every architecture rule on the workspace at `root`: the dependency graph
/// against `arch.toml`, then every core's sources against [`sources::check_source`]
/// and every library crate's against [`sources::check_library_source`].
///
/// # Errors
///
/// Fails when `arch.toml`, `cargo metadata`, git or a scanned source cannot be read.
pub fn check_workspace(root: &Path) -> Result<Vec<Finding>, ArchError> {
    let (config, graph) = load(root)?;
    let mut findings = check_graph(&config, &graph);
    findings.extend(check_sources(root, &scanned_crates(&config, &graph))?);
    Ok(findings)
}

/// Lists the cores of the workspace at `root`: every member under the cores directory
/// that `arch.toml` names.
///
/// # Errors
///
/// Fails when `arch.toml` or `cargo metadata` cannot be read.
pub fn cores(root: &Path) -> Result<Vec<PackageNode>, ArchError> {
    let (config, graph) = load(root)?;
    Ok(core_packages(&config, &graph).cloned().collect())
}

fn load(root: &Path) -> Result<(ArchConfig, WorkspaceGraph), ArchError> {
    let text = std::fs::read_to_string(root.join("arch.toml"))
        .map_err(|error| ArchError(format!("cannot read arch.toml: {error}")))?;
    let config =
        ArchConfig::parse(&text).map_err(|error| ArchError(format!("arch.toml: {error}")))?;
    Ok((config, load_graph(root)?))
}

fn core_packages<'a>(
    config: &'a ArchConfig,
    graph: &'a WorkspaceGraph,
) -> impl Iterator<Item = &'a PackageNode> {
    graph
        .packages
        .iter()
        .filter(|package| is_core(config, package))
}

fn is_core(config: &ArchConfig, package: &PackageNode) -> bool {
    package.manifest_dir.starts_with(&config.cores.directory)
}

/// The directory of every crate whose sources are read, with the check its role gets.
fn scanned_crates<'a>(
    config: &ArchConfig,
    graph: &'a WorkspaceGraph,
) -> Vec<(&'a Path, SourceCheck)> {
    graph
        .packages
        .iter()
        .filter_map(|package| {
            let check = source_check(role_of(config, package)?)?;
            Some((package.manifest_dir.as_path(), check))
        })
        .collect()
}

/// The source check of `role`: only a core's and a library crate's sources are read.
fn source_check(role: Role) -> Option<SourceCheck> {
    match role {
        Role::Core => Some(sources::check_source),
        Role::Library => Some(sources::check_library_source),
        Role::CompositionRoot | Role::Adapter | Role::Tooling => None,
    }
}

fn check_sources(root: &Path, scanned: &[(&Path, SourceCheck)]) -> Result<Vec<Finding>, ArchError> {
    let files = git::rust_files(root).map_err(|error| ArchError(format!("git: {error}")))?;
    let mut findings = Vec::new();
    for file in &files {
        let Some((_, check)) = scanned.iter().find(|(dir, _)| file.starts_with(dir)) else {
            continue;
        };
        let source = std::fs::read_to_string(root.join(file))
            .map_err(|error| ArchError(format!("{}: {error}", file.display())))?;
        let found = check(file, &source)
            .map_err(|error| ArchError(format!("{}: {error}", file.display())))?;
        findings.extend(found);
    }
    Ok(findings)
}

/// Reads the workspace members and their dependencies with `cargo metadata`.
///
/// # Errors
///
/// Fails when `cargo metadata` fails, for example outside a Cargo workspace.
pub fn load_graph(root: &Path) -> Result<WorkspaceGraph, ArchError> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .current_dir(root)
        .no_deps()
        .exec()
        .map_err(|error| ArchError(format!("cargo metadata: {error}")))?;
    let members: Vec<String> = metadata
        .packages
        .iter()
        .map(|package| package.name.to_string())
        .collect();
    let workspace_root = metadata.workspace_root.as_std_path();
    let packages = metadata
        .packages
        .iter()
        .map(|package| package_node(package, &members, workspace_root))
        .collect();
    Ok(WorkspaceGraph { packages })
}

fn package_node(
    package: &cargo_metadata::Package,
    members: &[String],
    workspace_root: &Path,
) -> PackageNode {
    PackageNode {
        name: package.name.to_string(),
        manifest_dir: package
            .manifest_path
            .as_std_path()
            .parent()
            .and_then(|dir| dir.strip_prefix(workspace_root).ok())
            .map(Path::to_path_buf)
            .unwrap_or_default(),
        inherits_workspace_lints: inherits_workspace_lints(package.manifest_path.as_std_path()),
        dependencies: package
            .dependencies
            .iter()
            .map(|dependency| DependencyEdge {
                name: dependency.name.clone(),
                kind: kind_of(dependency.kind),
                workspace_member: members.contains(&dependency.name),
            })
            .collect(),
    }
}

fn inherits_workspace_lints(manifest: &Path) -> bool {
    std::fs::read_to_string(manifest)
        .ok()
        .and_then(|text| text.parse::<toml::Table>().ok())
        .and_then(|table| table.get("lints")?.get("workspace")?.as_bool())
        == Some(true)
}

fn kind_of(kind: cargo_metadata::DependencyKind) -> DependencyKind {
    match kind {
        cargo_metadata::DependencyKind::Development => DependencyKind::Dev,
        cargo_metadata::DependencyKind::Build => DependencyKind::Build,
        cargo_metadata::DependencyKind::Normal | cargo_metadata::DependencyKind::Unknown => {
            DependencyKind::Normal
        }
    }
}

/// Checks the dependency graph against the rules in `config`.
#[must_use]
pub fn check_graph(config: &ArchConfig, graph: &WorkspaceGraph) -> Vec<Finding> {
    let roles: HashMap<&str, Role> = graph
        .packages
        .iter()
        .filter_map(|package| role_of(config, package).map(|role| (package.name.as_str(), role)))
        .collect();
    let mut findings = config_findings(config, graph, &roles);
    for package in &graph.packages {
        let path = manifest_path(&package.manifest_dir);
        let messages = package_messages(config, package, &roles);
        findings.extend(messages.into_iter().map(|message| Finding {
            path: path.clone(),
            line: 1,
            message,
        }));
    }
    findings
}

/// The manifest in `manifest_dir`, written with `/` on every platform, so a finding
/// reads the same wherever the check runs.
fn manifest_path(manifest_dir: &Path) -> PathBuf {
    let path = manifest_dir.join("Cargo.toml");
    PathBuf::from(path.to_string_lossy().replace('\\', "/"))
}

fn package_messages(
    config: &ArchConfig,
    package: &PackageNode,
    roles: &HashMap<&str, Role>,
) -> Vec<String> {
    let name = &package.name;
    let role = roles.get(name.as_str()).copied();
    let mut messages = match role {
        None => vec![no_role_message(config, name)],
        Some(role) => dependency_messages(config, package, role, roles),
    };
    if let Some(listed) = role.and_then(|role| allow_listed(config, role))
        && !listed.rules.allowed_dependencies.contains_key(name)
    {
        messages.push(format!(
            "{} `{name}` has no allow-list in arch.toml: add `{name} = []` under \
             `[{}.allowed-dependencies]`",
            listed.noun, listed.table
        ));
    }
    if !package.inherits_workspace_lints {
        messages.push(format!(
            "`{name}` must set `[lints] workspace = true` in its manifest"
        ));
    }
    messages
}

/// What a crate that takes no role is told: the roles `arch.toml` names by crate, and
/// the directories whose crates take a role.
fn no_role_message(config: &ArchConfig, name: &str) -> String {
    let libraries = config
        .libraries
        .as_ref()
        .map(|libraries| format!(", {}", libraries.directory.display()))
        .unwrap_or_default();
    format!(
        "`{name}` has no role in arch.toml: name it as a composition root or tooling, or \
         move it under {}{libraries} or {}",
        config.cores.directory.display(),
        config.adapters.directory.display()
    )
}

fn config_findings(
    config: &ArchConfig,
    graph: &WorkspaceGraph,
    roles: &HashMap<&str, Role>,
) -> Vec<Finding> {
    let tables = [Role::Core, Role::Library]
        .into_iter()
        .filter_map(|role| allow_listed(config, role))
        .flat_map(|listed| table_messages(&listed, roles));
    missing_crates(config, graph)
        .into_iter()
        .chain(tables)
        .chain(missing_reasons(config))
        .map(|message| Finding {
            path: PathBuf::from("arch.toml"),
            line: 1,
            message,
        })
        .collect()
}

/// The table that holds the allow-lists of `role`'s crates, for the roles that have
/// one: the cores, and the library crates once `arch.toml` names their directory.
fn allow_listed(config: &ArchConfig, role: Role) -> Option<AllowListed<'_>> {
    match role {
        Role::Core => Some(AllowListed {
            role,
            noun: "core",
            table: "cores",
            rules: &config.cores,
        }),
        Role::Library => config.libraries.as_ref().map(|rules| AllowListed {
            role,
            noun: "library crate",
            table: "libraries",
            rules,
        }),
        Role::CompositionRoot | Role::Adapter | Role::Tooling => None,
    }
}

/// What is wrong with the `[cores]` or the `[libraries]` table: a directory that holds
/// no crate, so the role's rules would check nothing, and allow-lists that name no
/// crate of the role.
fn table_messages(listed: &AllowListed<'_>, roles: &HashMap<&str, Role>) -> Vec<String> {
    let directory = listed.rules.directory.display();
    let has_the_role = |name: &str| roles.get(name) == Some(&listed.role);
    let empty = (!roles.keys().any(|name| has_the_role(name))).then(|| {
        format!(
            "arch.toml names the {} directory `{directory}`, which holds no crate",
            listed.table
        )
    });
    let stray = listed
        .rules
        .allowed_dependencies
        .keys()
        .filter(|name| !has_the_role(name))
        .map(|name| {
            format!(
                "arch.toml holds an allow-list for `{name}`, which is not a {} under \
                 {directory}",
                listed.noun
            )
        });
    empty.into_iter().chain(stray).collect()
}

fn missing_crates(config: &ArchConfig, graph: &WorkspaceGraph) -> Vec<String> {
    config
        .composition_roots
        .crates
        .iter()
        .chain(&config.tooling.crates)
        .filter(|name| !graph.packages.iter().any(|package| package.name == **name))
        .map(|name| format!("arch.toml names `{name}`, which is not a workspace member"))
        .collect()
}

fn missing_reasons(config: &ArchConfig) -> Vec<String> {
    let allowed = std::iter::once(&config.cores)
        .chain(&config.libraries)
        .flat_map(|rules| &rules.allowed_dependencies)
        .flat_map(|(owner, entries)| entries.iter().map(move |entry| (owner, entry)))
        .filter(|(_, entry)| entry.reason.trim().is_empty())
        .map(|(owner, entry)| {
            format!(
                "allow-list entry `{}` of `{owner}` needs a reason",
                entry.name
            )
        });
    let couplings = config
        .adapters
        .couplings
        .iter()
        .filter(|coupling| coupling.reason.trim().is_empty())
        .map(|coupling| {
            format!(
                "coupling `{}` -> `{}` needs a reason",
                coupling.from, coupling.to
            )
        });
    allowed.chain(couplings).collect()
}

fn role_of(config: &ArchConfig, package: &PackageNode) -> Option<Role> {
    let in_the_libraries =
        |libraries: &AllowListedCrates| package.manifest_dir.starts_with(&libraries.directory);
    if is_core(config, package) {
        Some(Role::Core)
    } else if config.libraries.as_ref().is_some_and(in_the_libraries) {
        Some(Role::Library)
    } else if config.composition_roots.crates.contains(&package.name) {
        Some(Role::CompositionRoot)
    } else if config.tooling.crates.contains(&package.name) {
        Some(Role::Tooling)
    } else if package.manifest_dir.starts_with(&config.adapters.directory) {
        Some(Role::Adapter)
    } else {
        None
    }
}

fn dependency_messages(
    config: &ArchConfig,
    package: &PackageNode,
    role: Role,
    roles: &HashMap<&str, Role>,
) -> Vec<String> {
    package
        .dependencies
        .iter()
        .filter(|dependency| dependency.name != package.name)
        .filter_map(|dependency| {
            let target = roles.get(dependency.name.as_str()).copied();
            let edge = DependencyUse {
                from: &package.name,
                role,
                dependency,
                target,
            };
            violation(config, &edge)
        })
        .collect()
}

/// One dependency declared by a crate whose role is known.
struct DependencyUse<'a> {
    from: &'a str,
    role: Role,
    dependency: &'a DependencyEdge,
    target: Option<Role>,
}

fn violation(config: &ArchConfig, edge: &DependencyUse<'_>) -> Option<String> {
    if edge.dependency.workspace_member {
        workspace_violation(config, edge)
    } else {
        external_violation(config, edge)
    }
}

/// What is wrong with a dependency on a crate outside the workspace: a core or a
/// library crate may use only the crates on its own allow-list.
fn external_violation(config: &ArchConfig, edge: &DependencyUse<'_>) -> Option<String> {
    let listed = allow_listed(config, edge.role)?;
    let to = &edge.dependency.name;
    (!is_allowed(listed.rules, edge.from, to)).then(|| {
        format!(
            "{} depends on `{to}`, which is not on its allow-list in arch.toml",
            listed.noun
        )
    })
}

/// What is wrong with a dependency on another workspace crate, by the two roles.
fn workspace_violation(config: &ArchConfig, edge: &DependencyUse<'_>) -> Option<String> {
    let (from, to) = (edge.from, &edge.dependency.name);
    match (edge.role, edge.target) {
        (Role::Library, _) => Some(format!(
            "library crate depends on workspace crate `{to}`; a library crate depends on no workspace crate"
        )),
        (Role::Core, Some(Role::Library)) | (Role::Adapter, Some(Role::Core | Role::Library)) => None,
        (Role::Core, _) => Some(format!(
            "core depends on workspace crate `{to}`; a core depends on no workspace crate but library crates"
        )),
        (Role::Adapter, _) => (!is_coupled(config, from, to)).then(|| {
            format!("adapter `{from}` depends on `{to}`; adapters depend only on cores, library crates and couplings declared in arch.toml")
        }),
        (Role::Tooling, Some(Role::Adapter)) => {
            Some(format!("`{from}` depends on adapter `{to}`; only composition roots wire adapters"))
        }
        (Role::CompositionRoot, Some(Role::CompositionRoot)) => {
            Some(format!("composition root `{from}` depends on composition root `{to}`"))
        }
        (Role::CompositionRoot, Some(Role::Tooling)) => {
            Some(format!("`{from}` depends on tooling crate `{to}`; product crates do not depend on tooling"))
        }
        (Role::Tooling | Role::CompositionRoot, _) => None,
    }
}

/// Whether `dependency` is on the allow-list of `owner`, and of that crate only.
fn is_allowed(rules: &AllowListedCrates, owner: &str, dependency: &str) -> bool {
    rules
        .allowed_dependencies
        .get(owner)
        .is_some_and(|entries| entries.iter().any(|entry| entry.name == dependency))
}

fn is_coupled(config: &ArchConfig, from: &str, to: &str) -> bool {
    config
        .adapters
        .couplings
        .iter()
        .any(|coupling| coupling.from == from && coupling.to == to)
}
