// use crate::game::Solution;
use anyhow::{anyhow, Context, Result};
use axum::{extract::Query, response::Json, routing::get, Router};
use config::COOKIE_KEY;
use serde_json::{json, Value};
use std::{collections::HashMap, net::SocketAddr};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use crate::config::{init_cookie_key, init_env_vars};
use crate::dictionary::{load_dictionary, DICTIONARY};

mod config;
mod dictionary;
mod game;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // let a = ['t', 'n', 'w', 'u'];
    // let b = ['m', 'y', 'o', 'p'];
    // let c = ['q', 'a', 'i', 's'];
    // let mut solution = Solution::new([a, b, c]);
    // solution.add_letter((0, 0));
    // solution.add_letter((2, 1));
    // solution.add_letter((0, 1));
    // solution.enter_word();
    // println!("{solution:#?}");

    // ---------------

    dotenvy::dotenv().with_context(|| "Error while reading dotenv config.")?;

    init_env_vars()?;
    init_cookie_key()?;

    DICTIONARY
        .set(load_dictionary("./data/words.txt")?)
        .map_err(|_| anyhow!("Error while loading dictionary."))?;

    let app = Router::new()
        .route("/", get(handler))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .with_context(|| "Error while starting Axum server")?;

    Ok(())
}

async fn handler(Query(params): Query<HashMap<String, String>>, cookies: Cookies) -> Json<Value> {
    let key = COOKIE_KEY.get();

    if let None = key {
        return Json(json!({
            "error": "Could not decrypt cookies.",
        }));
    }

    let key = key.unwrap();
    let private_cookies = cookies.private(key);

    let state_cookie = private_cookies.get("triangulettered_state");

    Json(json!(params))
}
