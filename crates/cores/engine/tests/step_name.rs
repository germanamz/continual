//! Behaviour of [`StepName`], the name a step has in `continual.toml`: it accepts and
//! rejects through the name rule of `continual-vocabulary`, whose own tests hold every
//! case of the rule.

use continual_engine::workspace::StepName;
use continual_vocabulary::NameError::MisplacedHyphen;

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
fn consecutive_hyphens_are_rejected() {
    assert_eq!(
        StepName::parse("fmt--edited"),
        Err(MisplacedHyphen { index: 4 })
    );
}
