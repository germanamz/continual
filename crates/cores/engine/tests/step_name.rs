//! Behaviour of [`StepName`], the name a step has in `continual.toml`.

use continual_engine::workspace::StepNameError::{
    Empty, ForbiddenCharacter, InvalidStart, MisplacedHyphen,
};
use continual_engine::workspace::{StepName, StepNameError};

fn text_of(input: &str) -> String {
    match StepName::parse(input) {
        Ok(name) => name.as_str().to_owned(),
        Err(error) => panic!("{input:?} should parse, got: {error}"),
    }
}

#[test]
fn a_lowercase_word_is_a_step_name() {
    assert_eq!(text_of("fmt"), "fmt");
}

#[test]
fn single_hyphens_join_words() {
    assert_eq!(text_of("fmt-edited"), "fmt-edited");
}

#[test]
fn digits_may_follow_the_first_letter() {
    assert_eq!(text_of("rust2024-edition"), "rust2024-edition");
}

#[test]
fn a_single_letter_is_a_step_name() {
    assert_eq!(text_of("a"), "a");
}

#[test]
fn empty_input_is_rejected() {
    assert_eq!(StepName::parse(""), Err(Empty));
}

#[test]
fn a_name_must_start_with_a_lowercase_letter() {
    for character in ['2', '-', 'A', 'é'] {
        let input = format!("{character}lint");
        assert_eq!(StepName::parse(&input), Err(InvalidStart { character }));
    }
}

#[test]
fn characters_other_than_lowercase_letters_digits_and_hyphens_are_rejected() {
    for character in ['A', '_', ' ', '/', '.', ':', 'é', '\n'] {
        let input = format!("ab{character}c");
        assert_eq!(
            StepName::parse(&input),
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
        StepName::parse("aéb"),
        Err(ForbiddenCharacter {
            index: 1,
            character: 'é'
        })
    );
    assert_eq!(
        StepName::parse("a-bé_"),
        Err(ForbiddenCharacter {
            index: 3,
            character: 'é'
        })
    );
}

#[test]
fn consecutive_hyphens_are_rejected() {
    assert_eq!(
        StepName::parse("fmt--edited"),
        Err(MisplacedHyphen { index: 4 })
    );
}

#[test]
fn a_trailing_hyphen_is_rejected() {
    assert_eq!(StepName::parse("lint-"), Err(MisplacedHyphen { index: 4 }));
}

#[test]
fn the_first_offending_character_from_the_left_is_reported() {
    assert_eq!(StepName::parse("a--B"), Err(MisplacedHyphen { index: 2 }));
    assert_eq!(
        StepName::parse("aB--"),
        Err(ForbiddenCharacter {
            index: 1,
            character: 'B'
        })
    );
}

#[test]
fn errors_explain_themselves() {
    assert_eq!(Empty.to_string(), "step name is empty");
    assert_eq!(
        InvalidStart { character: '2' }.to_string(),
        "step name starts with '2', not a lowercase letter"
    );
    assert_eq!(
        ForbiddenCharacter {
            index: 3,
            character: '_'
        }
        .to_string(),
        "character 3 of the step name is '_', not a lowercase letter, digit or hyphen"
    );
    assert_eq!(
        MisplacedHyphen { index: 4 }.to_string(),
        "the hyphen at character 4 of the step name is not between two letters or digits"
    );
}

#[test]
fn errors_are_standard_errors() {
    let error: &dyn core::error::Error = &StepNameError::Empty;
    assert!(error.source().is_none());
}
