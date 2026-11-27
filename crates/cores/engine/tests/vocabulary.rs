//! The engine speaks the records of `continual-vocabulary`: a configuration reaches it
//! with them, and a problem comes back located with them.

use std::collections::BTreeMap;

use continual_engine::workspace::{ConfigInput, Workspace};
use continual_vocabulary::{Located, Location};

#[test]
fn a_problem_is_located_with_the_shared_location_its_item_came_with() {
    let location = Location {
        file: "continual.toml".to_owned(),
        line: 2,
    };
    let jobs = Located {
        value: 0,
        location: location.clone(),
    };
    let input = ConfigInput {
        jobs: Some(jobs),
        timeout: None,
        steps: BTreeMap::new(),
    };
    let problems = Workspace::build(input).expect_err("`jobs` is below 1");
    let locations: Vec<Location> = problems
        .into_iter()
        .map(|problem| problem.location)
        .collect();
    assert_eq!(locations, [location]);
}
