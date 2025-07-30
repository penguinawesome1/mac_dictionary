use mac_dictionary::{CliError, dictionary};
use std::path::Path;

dictionary! {
    r#type: u64,
    id = 30,
    wow = 3,
    binary = 1,
    ctx = 5,
}

#[test]
fn test_word_access() {
    let word: Word = Word::new(1000, 0, true, 3);

    assert_eq!(word.id(), 1000);
    assert_eq!(word.wow(), 0);
    assert!(word.binary());
    assert_eq!(word.ctx(), 3);
}

#[test]
fn test_initialize_dictionary() -> Result<(), CliError> {
    initialize_dictionary(Path::new("./tests/Words.toml"))?;

    let a: &Word = definition(0);
    let c: &Word = definition(2);

    println!("{}", c);

    assert!(a.binary());
    assert_eq!(c.id(), 2000);

    Ok(())
}
