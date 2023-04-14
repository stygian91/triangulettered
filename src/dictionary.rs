use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) static DICTIONARY: OnceCell<HashMap<String, ()>> = OnceCell::new();

pub(crate) fn word_exists(word: &str) -> bool {
    DICTIONARY.get().unwrap().contains_key(word)
}

pub(crate) fn load_dictionary(path: &str) -> Result<HashMap<String, ()>> {
    let contents = read_to_string(path).with_context(|| "Error while reading dictionary.")?;

    Ok(contents
        .lines()
        .map(|line| (line.to_owned(), ()))
        .collect::<HashMap<String, ()>>())
}
