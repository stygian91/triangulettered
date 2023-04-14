use anyhow::{Context, Result};

pub(crate) fn hex2bytes(input: &str) -> Result<Vec<u8>> {
    let mut res = vec![];

    for ch in input.chars() {
        res.push(
            ch.to_digit(16)
                .with_context(|| "Error while parsing hex.")? as u8,
        );
    }

    Ok(res)
}
