//! Behaviour of [`CheckName`], the name that identifies a check in a pipeline.

use continual_core::CheckNameError::{Empty, ForbiddenCharacter, InvalidStart, MisplacedHyphen};
use continual_core::{CheckName, CheckNameError};

fn text_of(input: &str) -> String {
    match CheckName::parse(input) {
        Ok(name) => name.as_str().to_owned(),
        Err(error) => panic!("{input:?} should parse, got: {error}"),
    }
}

#[test]
fn a_lowercase_word_is_a_check_name() {
    assert_eq!(text_of("comments"), "comments");
}

#[test]
fn single_hyphens_join_words() {
    assert_eq!(text_of("test-changes"), "test-changes");
}

#[test]
fn digits_may_follow_the_first_letter() {
    assert_eq!(text_of("rust2024-edition"), "rust2024-edition");
}

#[test]
fn a_single_letter_is_a_check_name() {
    assert_eq!(text_of("a"), "a");
}

#[test]
fn display_round_trips_the_parsed_text() {
    for input in ["arch", "lint-2"] {
        let name = CheckName::parse(input).unwrap();
        assert_eq!(name.to_string(), input);
    }
}

#[test]
fn from_str_agrees_with_parse() {
    assert_eq!("arch".parse::<CheckName>(), CheckName::parse("arch"));
    assert_eq!("".parse::<CheckName>(), CheckName::parse(""));
}

#[test]
fn empty_input_is_rejected() {
    assert_eq!(CheckName::parse(""), Err(Empty));
}

#[test]
fn a_name_must_start_with_a_lowercase_letter() {
    for character in ['2', '-', 'A', 'é'] {
        let input = format!("{character}lint");
        assert_eq!(CheckName::parse(&input), Err(InvalidStart { character }));
    }
}

#[test]
fn characters_other_than_lowercase_letters_digits_and_hyphens_are_rejected() {
    for character in ['A', '_', ' ', '/', '.', 'é', '\n'] {
        let input = format!("ab{character}c");
        assert_eq!(
            CheckName::parse(&input),
            Err(ForbiddenCharacter {
                index: 2,
                character
            })
        );
    }
}

#[test]
fn positions_count_characters_not_bytes() {
    assert_eq!(
        CheckName::parse("aéb"),
        Err(ForbiddenCharacter {
            index: 1,
            character: 'é'
        })
    );
    assert_eq!(
        CheckName::parse("a-bé_"),
        Err(ForbiddenCharacter {
            index: 3,
            character: 'é'
        })
    );
}

#[test]
fn consecutive_hyphens_are_rejected() {
    assert_eq!(
        CheckName::parse("test--changes"),
        Err(MisplacedHyphen { index: 5 })
    );
}

#[test]
fn a_trailing_hyphen_is_rejected() {
    assert_eq!(CheckName::parse("lint-"), Err(MisplacedHyphen { index: 4 }));
}

#[test]
fn the_first_problem_from_the_left_is_reported() {
    assert_eq!(CheckName::parse("a--B"), Err(MisplacedHyphen { index: 2 }));
    assert_eq!(
        CheckName::parse("aB--"),
        Err(ForbiddenCharacter {
            index: 1,
            character: 'B'
        })
    );
}

#[test]
fn errors_explain_themselves() {
    assert_eq!(Empty.to_string(), "check name is empty");
    assert_eq!(
        InvalidStart { character: '2' }.to_string(),
        "check name starts with '2', not a lowercase letter"
    );
    assert_eq!(
        ForbiddenCharacter {
            index: 3,
            character: '_'
        }
        .to_string(),
        "character 3 of the check name is '_', not a lowercase letter, digit or hyphen"
    );
    assert_eq!(
        MisplacedHyphen { index: 4 }.to_string(),
        "the hyphen at character 4 of the check name is not between two letters or digits"
    );
}

#[test]
fn errors_are_standard_errors() {
    let error: &dyn core::error::Error = &CheckNameError::Empty;
    assert!(error.source().is_none());
}
