//! Behaviour of [`Location`], where something is written.

use continual_vocabulary::Location;

#[test]
fn a_location_prints_as_its_file_and_line() {
    let location = Location {
        file: "src/lib.rs".to_owned(),
        line: 12,
    };
    assert_eq!(location.to_string(), "src/lib.rs:12");
}
