//! Behaviour of `cargo xtask arch`, the dependency and boundary rules.
#![expect(
    clippy::expect_used,
    reason = "test helpers fail the test when setup fails"
)]

use std::path::{Path, PathBuf};

use xtask::arch::sources::check_library_source;
use xtask::arch::sources::check_source;
use xtask::arch::{
    ArchConfig, DependencyEdge, DependencyKind, PackageNode, WorkspaceGraph, check_graph,
    check_workspace, cores, load_graph,
};

const CONFIG: &str = r#"
[cores]
directory = "crates/cores"

[cores.allowed-dependencies]
continual-core = [{ name = "itoa", reason = "pure number formatting" }]

[composition-roots]
crates = ["continual", "continual-ui"]

[adapters]
directory = "crates/adapters"
couplings = [{ from = "continual-outbox", to = "continual-store", reason = "shares its transaction" }]

[tooling]
crates = ["xtask"]
"#;

fn config() -> ArchConfig {
    ArchConfig::parse(CONFIG).expect("valid config")
}

fn package(name: &str, dir: &str, dependencies: &[(&str, DependencyKind, bool)]) -> PackageNode {
    PackageNode {
        name: name.to_owned(),
        manifest_dir: PathBuf::from(dir),
        inherits_workspace_lints: true,
        dependencies: dependencies
            .iter()
            .map(|(dependency, kind, workspace_member)| DependencyEdge {
                name: (*dependency).to_owned(),
                kind: *kind,
                workspace_member: *workspace_member,
            })
            .collect(),
    }
}

fn base_packages() -> Vec<PackageNode> {
    vec![
        package("continual-core", "crates/cores/core", &[]),
        package(
            "continual",
            "crates/continual",
            &[("continual-core", DependencyKind::Normal, true)],
        ),
        package("continual-ui", "crates/continual-ui", &[]),
        package("xtask", "xtask", &[]),
    ]
}

fn messages(extra: Vec<PackageNode>) -> Vec<String> {
    let mut packages = base_packages();
    packages.extend(extra);
    check_graph(&config(), &WorkspaceGraph { packages })
        .into_iter()
        .map(|finding| finding.message)
        .collect()
}

fn with_core(dependencies: &[(&str, DependencyKind, bool)]) -> Vec<String> {
    let mut packages = base_packages();
    packages[0] = package("continual-core", "crates/cores/core", dependencies);
    check_graph(&config(), &WorkspaceGraph { packages })
        .into_iter()
        .map(|finding| finding.message)
        .collect()
}

fn none() -> Vec<String> {
    Vec::new()
}

#[test]
fn the_real_workspace_passes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root");
    let text = std::fs::read_to_string(root.join("arch.toml")).expect("arch.toml");
    let config = ArchConfig::parse(&text).expect("valid arch.toml");
    let graph = load_graph(root).expect("cargo metadata");
    assert_eq!(check_graph(&config, &graph), Vec::new());
    assert_eq!(check_workspace(root).expect("check runs"), Vec::new());
}

#[test]
fn a_clean_graph_has_no_findings() {
    assert_eq!(messages(Vec::new()), none());
}

#[test]
fn an_unclassified_crate_fails() {
    let found = messages(vec![package("stray", "crates/stray", &[])]);
    assert_eq!(found.len(), 1);
    assert!(found[0].contains("`stray`"), "{found:?}");
}

#[test]
fn a_missing_configured_crate_fails() {
    let mut packages = base_packages();
    packages.retain(|package| package.name != "continual-ui");
    let found = check_graph(&config(), &WorkspaceGraph { packages });
    assert_eq!(found.len(), 1);
    assert!(found[0].message.contains("`continual-ui`"));
    assert_eq!(found[0].path, Path::new("arch.toml"));
}

#[test]
fn core_must_not_depend_on_workspace_crates() {
    let found = with_core(&[("continual", DependencyKind::Normal, true)]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("core depends on workspace crate `continual`"));
}

#[test]
fn core_external_dependency_needs_an_allow_list_entry() {
    for kind in [
        DependencyKind::Normal,
        DependencyKind::Dev,
        DependencyKind::Build,
    ] {
        let found = with_core(&[("serde", kind, false)]);
        assert_eq!(found.len(), 1, "{kind:?}");
        assert!(found[0].contains("`serde`") && found[0].contains("allow-list"));
    }
    assert_eq!(
        with_core(&[("itoa", DependencyKind::Normal, false)]),
        none()
    );
}

#[test]
fn allow_list_entries_need_a_reason() {
    let text = CONFIG.replace("pure number formatting", " ");
    let config = ArchConfig::parse(&text).expect("valid config");
    let found = check_graph(
        &config,
        &WorkspaceGraph {
            packages: base_packages(),
        },
    );
    assert_eq!(found.len(), 1);
    assert!(found[0].message.contains("`itoa`") && found[0].message.contains("reason"));
}

#[test]
fn couplings_need_a_reason() {
    let text = CONFIG.replace("shares its transaction", "");
    let config = ArchConfig::parse(&text).expect("valid config");
    let found = check_graph(
        &config,
        &WorkspaceGraph {
            packages: base_packages(),
        },
    );
    assert_eq!(found.len(), 1);
    assert!(found[0].message.contains("reason"));
}

#[test]
fn adapters_may_depend_on_the_core() {
    let adapter = package(
        "continual-store",
        "crates/adapters/store",
        &[
            ("continual-core", DependencyKind::Normal, true),
            ("serde", DependencyKind::Normal, false),
        ],
    );
    assert_eq!(messages(vec![adapter]), none());
}

#[test]
fn adapters_must_not_depend_on_each_other() {
    let store = package("continual-store", "crates/adapters/store", &[]);
    let http = package(
        "continual-http",
        "crates/adapters/http",
        &[("continual-store", DependencyKind::Dev, true)],
    );
    let found = messages(vec![store, http]);
    assert_eq!(found.len(), 1);
    assert!(found[0].contains("`continual-http`") && found[0].contains("`continual-store`"));
}

#[test]
fn a_declared_coupling_allows_one_adapter_to_use_another() {
    let store = package("continual-store", "crates/adapters/store", &[]);
    let outbox = package(
        "continual-outbox",
        "crates/adapters/outbox",
        &[("continual-store", DependencyKind::Normal, true)],
    );
    assert_eq!(messages(vec![store, outbox]), none());
}

#[test]
fn only_composition_roots_depend_on_adapters() {
    let mut packages = base_packages();
    packages.push(package("continual-store", "crates/adapters/store", &[]));
    packages[1].dependencies.push(DependencyEdge {
        name: "continual-store".to_owned(),
        kind: DependencyKind::Normal,
        workspace_member: true,
    });
    packages[3].dependencies.push(DependencyEdge {
        name: "continual-store".to_owned(),
        kind: DependencyKind::Normal,
        workspace_member: true,
    });
    let found = check_graph(&config(), &WorkspaceGraph { packages });
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].message.contains("`xtask`"));
}

#[test]
fn composition_roots_do_not_depend_on_each_other() {
    let mut packages = base_packages();
    packages[2].dependencies.push(DependencyEdge {
        name: "continual".to_owned(),
        kind: DependencyKind::Normal,
        workspace_member: true,
    });
    let found = check_graph(&config(), &WorkspaceGraph { packages });
    assert_eq!(found.len(), 1);
    assert!(
        found[0].message.contains("`continual-ui`") && found[0].message.contains("`continual`")
    );
}

#[test]
fn product_crates_do_not_depend_on_tooling() {
    let mut packages = base_packages();
    packages[1].dependencies.push(DependencyEdge {
        name: "xtask".to_owned(),
        kind: DependencyKind::Dev,
        workspace_member: true,
    });
    let found = check_graph(&config(), &WorkspaceGraph { packages });
    assert_eq!(found.len(), 1);
    assert!(found[0].message.contains("`xtask`"));
}

#[test]
fn findings_point_at_the_offending_manifest() {
    let found = check_graph(
        &config(),
        &WorkspaceGraph {
            packages: vec![package("stray", "crates/stray", &[])],
        },
    );
    assert!(
        found
            .iter()
            .any(|finding| finding.path == Path::new("crates/stray/Cargo.toml"))
    );
}

fn source_lines(source: &str) -> Vec<usize> {
    check_source(Path::new("core.rs"), source)
        .expect("valid Rust")
        .into_iter()
        .map(|finding| finding.line)
        .collect()
}

#[test]
fn core_sources_must_not_name_io_modules() {
    let source = "use std::fs;\nuse std::{env, io::Read};\nfn f() {\n    std::process::exit(1);\n}\nfn g(_: ::std::net::TcpStream) {}\nfn h() { std::thread::yield_now(); }\n";
    assert_eq!(source_lines(source), [1, 2, 2, 4, 6, 7]);
}

#[test]
fn other_std_and_core_modules_are_allowed() {
    let source = "use std::fmt;\nuse core::str::FromStr;\nfn f() -> std::collections::BTreeMap<u8, u8> { std::collections::BTreeMap::new() }\n";
    assert_eq!(source_lines(source), Vec::<usize>::new());
}

#[test]
fn findings_name_the_module_and_the_remedy() {
    let found = check_source(Path::new("core.rs"), "use std::fs::File;\n").expect("valid Rust");
    assert_eq!(found.len(), 1);
    assert!(found[0].message.contains("`std::fs`") && found[0].message.contains("port"));
}

#[test]
fn unparsable_sources_are_errors() {
    assert!(check_source(Path::new("core.rs"), "fn {").is_err());
}

fn xtask_arch(dir: &Path) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_xtask"))
        .current_dir(dir)
        .arg("arch")
        .output()
        .expect("xtask runs")
}

#[test]
fn the_command_passes_on_this_repository() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root");
    let output = xtask_arch(root);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn the_command_fails_with_code_2_without_arch_toml() {
    let dir = tempfile::tempdir().expect("temp dir");
    let output = xtask_arch(dir.path());
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("arch.toml"));
}

fn write_file(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(path, text).expect("write");
}

/// The `arch.toml` of a workspace whose cores live under `cores/`, each with an empty
/// allow-list.
fn arch_toml(cores: &[&str], tooling: &[&str]) -> String {
    let lists: Vec<String> = cores.iter().map(|core| format!("{core} = []\n")).collect();
    let tooling: Vec<String> = tooling.iter().map(|name| format!("\"{name}\"")).collect();
    format!(
        "[cores]\ndirectory = \"cores\"\n[cores.allowed-dependencies]\n{}[composition-roots]\ncrates = []\n[adapters]\ndirectory = \"crates/adapters\"\n[tooling]\ncrates = [{}]\n",
        lists.concat(),
        tooling.join(", ")
    )
}

/// A workspace under git with one crate for each `(directory, lib.rs source)`, named
/// for the last part of its directory, and `arch` as its `arch.toml`.
fn workspace(crates: &[(&str, &str)], arch: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("temp dir");
    let members: Vec<String> = crates
        .iter()
        .map(|(directory, _)| format!("\"{directory}\""))
        .collect();
    let manifest = format!(
        "[workspace]\nresolver = \"3\"\nmembers = [{}]\n\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\n",
        members.join(", ")
    );
    write_file(dir.path(), "Cargo.toml", &manifest);
    for (directory, source) in crates {
        let name = directory.rsplit('/').next().expect("a directory name");
        let manifest = format!(
            "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lints]\nworkspace = true\n"
        );
        write_file(dir.path(), &format!("{directory}/Cargo.toml"), &manifest);
        write_file(dir.path(), &format!("{directory}/src/lib.rs"), source);
    }
    write_file(dir.path(), "arch.toml", arch);
    let status = std::process::Command::new("git")
        .arg("-C")
        .arg(dir.path())
        .args(["init", "--quiet"])
        .status()
        .expect("git runs");
    assert!(status.success());
    dir
}

/// A one-crate workspace, under git, whose core `lib.rs` holds `source`.
fn workspace_with_core_source(source: &str) -> tempfile::TempDir {
    workspace(&[("cores/core", source)], &arch_toml(&["core"], &[]))
}

/// A workspace with two cores and a tooling crate, each `lib.rs` naming a forbidden
/// `std` module.
fn workspace_with_two_cores() -> tempfile::TempDir {
    workspace(
        &[
            ("cores/alpha", "use std::fs;\n"),
            ("cores/beta", "use std::net;\n"),
            ("tool", "use std::fs;\n"),
        ],
        &arch_toml(&["alpha", "beta"], &["tool"]),
    )
}

#[test]
fn the_workspace_check_reports_forbidden_imports_in_core_sources() {
    let dir = workspace_with_core_source("use std::fs;\n");
    let found: Vec<String> = check_workspace(dir.path())
        .expect("check runs")
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(
        found,
        [
            "cores/core/src/lib.rs:1: core must not use `std::fs`: reach the outside world through a port"
        ]
    );
}

#[test]
fn the_workspace_check_scans_the_sources_of_every_core_and_of_no_other_crate() {
    let dir = workspace_with_two_cores();
    let mut found: Vec<String> = check_workspace(dir.path())
        .expect("check runs")
        .iter()
        .map(ToString::to_string)
        .collect();
    found.sort();
    assert_eq!(
        found,
        [
            "cores/alpha/src/lib.rs:1: core must not use `std::fs`: reach the outside world through a port",
            "cores/beta/src/lib.rs:1: core must not use `std::net`: reach the outside world through a port",
        ]
    );
}

#[test]
fn the_cores_of_a_workspace_are_the_crates_under_its_cores_directory() {
    let dir = workspace_with_two_cores();
    let mut found: Vec<(String, PathBuf)> = cores(dir.path())
        .expect("cores load")
        .into_iter()
        .map(|core| (core.name, core.manifest_dir))
        .collect();
    found.sort();
    assert_eq!(
        found,
        [
            ("alpha".to_owned(), PathBuf::from("cores/alpha")),
            ("beta".to_owned(), PathBuf::from("cores/beta")),
        ]
    );
}

#[test]
fn the_cores_cannot_be_listed_without_arch_toml() {
    let dir = tempfile::tempdir().expect("temp dir");
    let error = cores(dir.path()).expect_err("no arch.toml");
    assert!(error.to_string().contains("arch.toml"), "{error}");
}

#[test]
fn a_crate_without_workspace_lints_fails() {
    let mut packages = base_packages();
    packages[1].inherits_workspace_lints = false;
    let found = check_graph(&config(), &WorkspaceGraph { packages });
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("`continual`")
            && found[0].message.contains("[lints] workspace = true")
    );
    assert_eq!(found[0].path, Path::new("crates/continual/Cargo.toml"));
}

#[test]
fn the_workspace_check_reads_each_crates_lints_table() {
    let dir = workspace_with_core_source("pub fn f() {}\n");
    std::fs::write(
        dir.path().join("cores/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write");
    let found: Vec<String> = check_workspace(dir.path())
        .expect("check runs")
        .into_iter()
        .map(|finding| finding.message)
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].contains("[lints] workspace = true"));
}

#[test]
fn a_crate_may_dev_depend_on_itself_to_enable_test_features() {
    assert_eq!(
        with_core(&[("continual-core", DependencyKind::Dev, true)]),
        none()
    );
}

/// The allow-list line of the second core in most tests below.
const ENGINE_LIST: &str = "continual-engine = []";

/// `CONFIG` with `list` as one more line under `[cores.allowed-dependencies]`.
fn config_with_list(list: &str) -> ArchConfig {
    let text = CONFIG.replace(
        "[composition-roots]",
        &format!("{list}\n\n[composition-roots]"),
    );
    ArchConfig::parse(&text).expect("valid config")
}

fn second_core(dependencies: &[(&str, DependencyKind, bool)]) -> PackageNode {
    package("continual-engine", "crates/cores/engine", dependencies)
}

/// The messages for the base packages plus `extra`, with `list` added to `CONFIG`.
fn messages_with_list(list: &str, extra: Vec<PackageNode>) -> Vec<String> {
    let mut packages = base_packages();
    packages.extend(extra);
    check_graph(&config_with_list(list), &WorkspaceGraph { packages })
        .into_iter()
        .map(|finding| finding.message)
        .collect()
}

#[test]
fn every_crate_under_the_cores_directory_is_a_core() {
    assert_eq!(
        messages_with_list(ENGINE_LIST, vec![second_core(&[])]),
        none()
    );
}

#[test]
fn a_cores_directory_without_crates_fails() {
    let text = CONFIG.replace(
        r#"continual-core = [{ name = "itoa", reason = "pure number formatting" }]"#,
        "",
    );
    let config = ArchConfig::parse(&text).expect("valid config");
    let mut packages = base_packages();
    packages.remove(0);
    let found = check_graph(&config, &WorkspaceGraph { packages });
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("`crates/cores`") && found[0].message.contains("no crate"),
        "{found:?}"
    );
    assert_eq!(found[0].path, Path::new("arch.toml"));
}

#[test]
fn no_core_depends_on_another_core_or_on_any_other_workspace_crate() {
    for target in ["continual-core", "continual", "xtask"] {
        let engine = second_core(&[(target, DependencyKind::Normal, true)]);
        let found = messages_with_list(ENGINE_LIST, vec![engine]);
        assert_eq!(found.len(), 1, "{target}: {found:?}");
        assert!(
            found[0].contains(&format!("core depends on workspace crate `{target}`")),
            "{found:?}"
        );
    }
}

#[test]
fn a_core_may_use_only_the_external_crates_on_its_own_allow_list() {
    let list = r#"continual-engine = [{ name = "memchr", reason = "pure byte search" }]"#;
    let own = second_core(&[("memchr", DependencyKind::Normal, false)]);
    assert_eq!(messages_with_list(list, vec![own]), none());
    let borrowed = second_core(&[("itoa", DependencyKind::Normal, false)]);
    let found = messages_with_list(list, vec![borrowed]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].contains("`itoa`") && found[0].contains("allow-list"),
        "{found:?}"
    );
}

#[test]
fn a_core_without_an_allow_list_of_its_own_fails() {
    let mut packages = base_packages();
    packages.push(second_core(&[]));
    let found = check_graph(&config(), &WorkspaceGraph { packages });
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("`continual-engine`") && found[0].message.contains("allow-list"),
        "{found:?}"
    );
    assert_eq!(found[0].path, Path::new("crates/cores/engine/Cargo.toml"));
}

#[test]
fn an_allow_list_must_name_a_core() {
    for stray in ["continual-engine", "continual"] {
        let config = config_with_list(&format!("{stray} = []"));
        let found = check_graph(
            &config,
            &WorkspaceGraph {
                packages: base_packages(),
            },
        );
        assert_eq!(found.len(), 1, "{stray}: {found:?}");
        assert!(
            found[0].message.contains(&format!("`{stray}`"))
                && found[0].message.contains("not a core"),
            "{found:?}"
        );
        assert_eq!(found[0].path, Path::new("arch.toml"));
    }
}

#[test]
fn every_cores_allow_list_entries_need_a_reason() {
    let list = r#"continual-engine = [{ name = "memchr", reason = "" }]"#;
    let engine = second_core(&[("memchr", DependencyKind::Normal, false)]);
    let found = messages_with_list(list, vec![engine]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].contains("`memchr`") && found[0].contains("reason"),
        "{found:?}"
    );
}

#[test]
fn an_adapter_may_depend_on_two_cores() {
    let bridge = package(
        "continual-engine-registry",
        "crates/adapters/engine-registry",
        &[
            ("continual-core", DependencyKind::Normal, true),
            ("continual-engine", DependencyKind::Normal, true),
        ],
    );
    assert_eq!(
        messages_with_list(ENGINE_LIST, vec![second_core(&[]), bridge]),
        none()
    );
}

/// The allow-list line of the library crate in most tests below.
const VOCABULARY_LIST: &str = "continual-vocabulary = []";

/// `CONFIG` with a `[libraries]` table that names `crates/libraries` and holds `lists`
/// under `[libraries.allowed-dependencies]`.
fn config_with_libraries(lists: &str) -> ArchConfig {
    let table = format!(
        "[libraries]\ndirectory = \"crates/libraries\"\n\n[libraries.allowed-dependencies]\n{lists}\n\n[composition-roots]"
    );
    let text = CONFIG.replace("[composition-roots]", &table);
    ArchConfig::parse(&text).expect("valid config")
}

fn vocabulary(dependencies: &[(&str, DependencyKind, bool)]) -> PackageNode {
    package(
        "continual-vocabulary",
        "crates/libraries/vocabulary",
        dependencies,
    )
}

/// The findings for the base packages plus `extra`, with a `[libraries]` table that
/// holds `lists`.
fn findings_with_libraries(lists: &str, extra: Vec<PackageNode>) -> Vec<xtask::finding::Finding> {
    let mut packages = base_packages();
    packages.extend(extra);
    check_graph(&config_with_libraries(lists), &WorkspaceGraph { packages })
}

/// The messages of [`findings_with_libraries`].
fn messages_with_libraries(lists: &str, extra: Vec<PackageNode>) -> Vec<String> {
    findings_with_libraries(lists, extra)
        .into_iter()
        .map(|finding| finding.message)
        .collect()
}

#[test]
fn without_a_libraries_table_no_crate_is_a_library_crate() {
    let found = messages(vec![vocabulary(&[])]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].contains("`continual-vocabulary` has no role")
            && found[0].ends_with("move it under crates/cores or crates/adapters"),
        "{found:?}"
    );
}

#[test]
fn every_crate_under_the_libraries_directory_is_a_library_crate() {
    let lists = format!("{VOCABULARY_LIST}\ncontinual-records = []");
    let records = package("continual-records", "crates/libraries/records", &[]);
    assert_eq!(
        messages_with_libraries(&lists, vec![vocabulary(&[]), records]),
        none()
    );
}

#[test]
fn a_crate_without_a_role_is_pointed_to_the_libraries_directory_too() {
    let stray = package("stray", "crates/stray", &[]);
    let found = messages_with_libraries(VOCABULARY_LIST, vec![vocabulary(&[]), stray]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].contains("`stray` has no role")
            && found[0]
                .ends_with("move it under crates/cores, crates/libraries or crates/adapters"),
        "{found:?}"
    );
}

#[test]
fn a_libraries_directory_without_crates_fails() {
    let found = findings_with_libraries("", Vec::new());
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0]
            .message
            .contains("the libraries directory `crates/libraries`")
            && found[0].message.contains("no crate"),
        "{found:?}"
    );
    assert_eq!(found[0].path, Path::new("arch.toml"));
}

#[test]
fn a_library_crate_without_an_allow_list_of_its_own_fails() {
    let records = package("continual-records", "crates/libraries/records", &[]);
    let found = findings_with_libraries(VOCABULARY_LIST, vec![vocabulary(&[]), records]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0]
            .message
            .contains("library crate `continual-records` has no allow-list")
            && found[0]
                .message
                .contains("`[libraries.allowed-dependencies]`"),
        "{found:?}"
    );
    assert_eq!(
        found[0].path,
        Path::new("crates/libraries/records/Cargo.toml")
    );
}

#[test]
fn a_libraries_allow_list_must_name_a_library_crate() {
    for stray in ["continual-records", "continual-core", "continual"] {
        let lists = format!("{VOCABULARY_LIST}\n{stray} = []");
        let found = findings_with_libraries(&lists, vec![vocabulary(&[])]);
        assert_eq!(found.len(), 1, "{stray}: {found:?}");
        assert!(
            found[0].message.contains(&format!("`{stray}`"))
                && found[0]
                    .message
                    .contains("not a library crate under crates/libraries"),
            "{found:?}"
        );
        assert_eq!(found[0].path, Path::new("arch.toml"));
    }
}

/// The base packages plus an adapter and a second library crate, with `vocabulary` as
/// the first library crate: one workspace crate of every role beside it.
fn findings_beside_every_role(vocabulary: PackageNode) -> Vec<xtask::finding::Finding> {
    let lists = format!("{VOCABULARY_LIST}\ncontinual-records = []");
    let store = package("continual-store", "crates/adapters/store", &[]);
    let records = package("continual-records", "crates/libraries/records", &[]);
    findings_with_libraries(&lists, vec![store, records, vocabulary])
}

#[test]
fn a_library_crate_depends_on_no_workspace_crate_of_any_role() {
    for target in [
        "continual-core",
        "continual-store",
        "continual",
        "xtask",
        "continual-records",
    ] {
        let found =
            findings_beside_every_role(vocabulary(&[(target, DependencyKind::Normal, true)]));
        assert_eq!(found.len(), 1, "{target}: {found:?}");
        assert!(
            found[0].message.contains(&format!(
                "library crate depends on workspace crate `{target}`"
            )),
            "{found:?}"
        );
        assert_eq!(
            found[0].path,
            Path::new("crates/libraries/vocabulary/Cargo.toml")
        );
    }
}

#[test]
fn a_library_crates_dev_and_build_dependencies_on_workspace_crates_count() {
    for kind in [DependencyKind::Dev, DependencyKind::Build] {
        let found = findings_beside_every_role(vocabulary(&[("continual-core", kind, true)]));
        assert_eq!(found.len(), 1, "{kind:?}: {found:?}");
        assert!(
            found[0]
                .message
                .contains("library crate depends on workspace crate `continual-core`"),
            "{found:?}"
        );
    }
}

#[test]
fn a_library_crate_may_dev_depend_on_itself_to_enable_test_features() {
    let own = vocabulary(&[("continual-vocabulary", DependencyKind::Dev, true)]);
    assert_eq!(messages_with_libraries(VOCABULARY_LIST, vec![own]), none());
}

/// An allow-list that lets the library crate use `memchr` and nothing else.
const VOCABULARY_MEMCHR_LIST: &str =
    r#"continual-vocabulary = [{ name = "memchr", reason = "pure byte search" }]"#;

#[test]
fn a_library_crate_may_use_only_the_external_crates_on_its_own_allow_list() {
    for kind in [
        DependencyKind::Normal,
        DependencyKind::Dev,
        DependencyKind::Build,
    ] {
        let own = vocabulary(&[("memchr", kind, false)]);
        assert_eq!(
            messages_with_libraries(VOCABULARY_MEMCHR_LIST, vec![own]),
            none(),
            "{kind:?}"
        );
        let borrowed = vocabulary(&[("itoa", kind, false)]);
        let found = messages_with_libraries(VOCABULARY_MEMCHR_LIST, vec![borrowed]);
        assert_eq!(found.len(), 1, "{kind:?}: {found:?}");
        assert!(
            found[0].contains("library crate depends on `itoa`") && found[0].contains("allow-list"),
            "{found:?}"
        );
    }
}

#[test]
fn a_library_crates_allow_list_entries_need_a_reason() {
    let lists = VOCABULARY_MEMCHR_LIST.replace("pure byte search", " ");
    let own = vocabulary(&[("memchr", DependencyKind::Normal, false)]);
    let found = findings_with_libraries(&lists, vec![own]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0]
            .message
            .contains("`memchr` of `continual-vocabulary`")
            && found[0].message.contains("reason"),
        "{found:?}"
    );
    assert_eq!(found[0].path, Path::new("arch.toml"));
}

/// `packages` with `dependency` added to the package named `from`.
fn depending(
    mut packages: Vec<PackageNode>,
    from: &str,
    dependency: (&str, DependencyKind),
) -> Vec<PackageNode> {
    let package = packages
        .iter_mut()
        .find(|package| package.name == from)
        .expect("a package of that name");
    package.dependencies.push(DependencyEdge {
        name: dependency.0.to_owned(),
        kind: dependency.1,
        workspace_member: true,
    });
    packages
}

/// The base packages with an adapter and the library crate.
fn packages_with_a_library_crate() -> Vec<PackageNode> {
    let mut packages = base_packages();
    packages.push(package("continual-store", "crates/adapters/store", &[]));
    packages.push(vocabulary(&[]));
    packages
}

#[test]
fn a_core_an_adapter_and_a_composition_root_may_each_depend_on_a_library_crate() {
    let config = config_with_libraries(VOCABULARY_LIST);
    for from in ["continual-core", "continual-store", "continual"] {
        for kind in [
            DependencyKind::Normal,
            DependencyKind::Dev,
            DependencyKind::Build,
        ] {
            let packages = depending(
                packages_with_a_library_crate(),
                from,
                ("continual-vocabulary", kind),
            );
            assert_eq!(
                check_graph(&config, &WorkspaceGraph { packages }),
                Vec::new(),
                "{from}, {kind:?}"
            );
        }
    }
}

#[test]
fn a_core_beside_a_library_crate_still_depends_on_no_other_workspace_crate() {
    let config = config_with_libraries(VOCABULARY_LIST);
    for target in ["continual-store", "continual", "xtask"] {
        let packages = depending(
            packages_with_a_library_crate(),
            "continual-core",
            (target, DependencyKind::Normal),
        );
        let found = check_graph(&config, &WorkspaceGraph { packages });
        assert_eq!(found.len(), 1, "{target}: {found:?}");
        assert!(
            found[0]
                .message
                .contains(&format!("core depends on workspace crate `{target}`"))
                && found[0].message.ends_with("but library crates"),
            "{found:?}"
        );
    }
}

#[test]
fn tooling_may_depend_on_a_library_crate() {
    let packages = depending(
        packages_with_a_library_crate(),
        "xtask",
        ("continual-vocabulary", DependencyKind::Normal),
    );
    let found = check_graph(
        &config_with_libraries(VOCABULARY_LIST),
        &WorkspaceGraph { packages },
    );
    assert_eq!(found, Vec::new());
}

#[test]
fn an_adapter_beside_a_library_crate_still_depends_on_no_other_adapter() {
    let mut packages = packages_with_a_library_crate();
    packages.push(package("continual-http", "crates/adapters/http", &[]));
    let packages = depending(
        packages,
        "continual-http",
        ("continual-store", DependencyKind::Normal),
    );
    let found = check_graph(
        &config_with_libraries(VOCABULARY_LIST),
        &WorkspaceGraph { packages },
    );
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0]
            .message
            .contains("adapter `continual-http` depends on `continual-store`")
            && found[0].message.contains("library crates"),
        "{found:?}"
    );
}

#[test]
fn a_library_crate_without_workspace_lints_fails() {
    let mut own = vocabulary(&[]);
    own.inherits_workspace_lints = false;
    let found = findings_with_libraries(VOCABULARY_LIST, vec![own]);
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].message.contains("`continual-vocabulary`")
            && found[0].message.contains("[lints] workspace = true"),
        "{found:?}"
    );
    assert_eq!(
        found[0].path,
        Path::new("crates/libraries/vocabulary/Cargo.toml")
    );
}

#[test]
fn library_crate_sources_must_not_name_io_modules() {
    let source = "use std::fs;\nuse std::{env, io::Read};\nfn f() {\n    std::process::exit(1);\n}\nfn g(_: ::std::net::TcpStream) {}\nfn h() { std::thread::yield_now(); }\n";
    let lines: Vec<usize> = check_library_source(Path::new("library.rs"), source)
        .expect("valid Rust")
        .into_iter()
        .map(|finding| finding.line)
        .collect();
    assert_eq!(lines, [1, 2, 2, 4, 6, 7]);
}

#[test]
fn other_std_and_core_modules_are_allowed_in_a_library_crate() {
    let source = "use std::fmt;\nuse core::str::FromStr;\nfn f() -> std::collections::BTreeMap<u8, u8> { std::collections::BTreeMap::new() }\n";
    let found = check_library_source(Path::new("library.rs"), source).expect("valid Rust");
    assert_eq!(found, Vec::new());
}

#[test]
fn a_library_crates_findings_name_the_module_and_the_crates_role() {
    let found =
        check_library_source(Path::new("library.rs"), "use std::fs::File;\n").expect("valid Rust");
    let found: Vec<String> = found.iter().map(ToString::to_string).collect();
    assert_eq!(
        found,
        [
            "library.rs:1: library crate must not use `std::fs`: it holds data and reaches nothing outside itself"
        ]
    );
}

#[test]
fn unparsable_library_crate_sources_are_errors() {
    assert!(check_library_source(Path::new("library.rs"), "fn {").is_err());
}

/// `arch_toml` with a `[libraries]` table that names `libraries/` and gives each of
/// `libraries` an empty allow-list.
fn arch_toml_with_libraries(cores: &[&str], libraries: &[&str], tooling: &[&str]) -> String {
    let lists: Vec<String> = libraries
        .iter()
        .map(|library| format!("{library} = []\n"))
        .collect();
    let table = format!(
        "[libraries]\ndirectory = \"libraries\"\n[libraries.allowed-dependencies]\n{}[composition-roots]",
        lists.concat()
    );
    arch_toml(cores, tooling).replace("[composition-roots]", &table)
}

#[test]
fn the_workspace_check_scans_a_library_crates_sources_as_it_scans_a_cores() {
    let dir = workspace(
        &[
            ("cores/core", "use std::net;\n"),
            ("libraries/vocabulary", "use std::fs;\n"),
            ("libraries/records", "use std::fmt;\n"),
            ("tool", "use std::fs;\n"),
        ],
        &arch_toml_with_libraries(&["core"], &["vocabulary", "records"], &["tool"]),
    );
    let mut found: Vec<String> = check_workspace(dir.path())
        .expect("check runs")
        .iter()
        .map(ToString::to_string)
        .collect();
    found.sort();
    assert_eq!(
        found,
        [
            "cores/core/src/lib.rs:1: core must not use `std::net`: reach the outside world through a port",
            "libraries/vocabulary/src/lib.rs:1: library crate must not use `std::fs`: it holds data and reaches nothing outside itself",
        ]
    );
}

#[test]
fn the_workspace_check_holds_a_library_crate_to_the_dependency_rules() {
    let dir = workspace(
        &[
            ("cores/core", "pub fn f() {}\n"),
            ("libraries/vocabulary", "pub fn f() {}\n"),
        ],
        &arch_toml_with_libraries(&["core"], &["vocabulary"], &[]),
    );
    let manifest = "[package]\nname = \"vocabulary\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ncore = { path = \"../../cores/core\" }\n\n[lints]\nworkspace = true\n";
    write_file(dir.path(), "libraries/vocabulary/Cargo.toml", manifest);
    let found: Vec<String> = check_workspace(dir.path())
        .expect("check runs")
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");
    assert!(
        found[0].starts_with(
            "libraries/vocabulary/Cargo.toml:1: library crate depends on workspace crate `core`"
        ),
        "{found:?}"
    );
}
