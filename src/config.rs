use anyhow::{anyhow, Context, Result};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use tower_cookies::Key;

use crate::utils::hex2bytes;

pub(crate) static COOKIE_KEY: OnceCell<Key> = OnceCell::new();
pub(crate) static ENV_VARS: OnceCell<HashMap<String, String>> = OnceCell::new();

pub(crate) fn init_cookie_key() -> Result<()> {
    COOKIE_KEY
        .set(Key::from(&get_secret()?))
        .map_err(|_| anyhow!("Error while setting cookie key."))?;

    Ok(())
}

pub(crate) fn init_env_vars() -> Result<()> {
    ENV_VARS
        .set(std::env::vars().collect::<HashMap<String, String>>())
        .map_err(|_| anyhow!("Error while setting env vars."))?;

    Ok(())
}

pub(crate) fn get_secret() -> Result<Vec<u8>> {
    let env_vars = ENV_VARS
        .get()
        .with_context(|| "Error while fetching env vars.")?;

    let secret = env_vars
        .get("TRIA_KEY")
        .with_context(|| "TRIA_KEY env variable not found.")?;

    hex2bytes(&secret).with_context(|| "Error while parsing session secret.")
}
