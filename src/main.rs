#![feature(with_options)]

#[macro_use]
extern crate clap;

use clap::{load_yaml, App, AppSettings::ColoredHelp, AppSettings::SubcommandRequired};
use spinners::{Spinners, Spinner};
use std::path::PathBuf;
use dirs::home_dir;
use std::fs::{create_dir, File};
use std::io::{Write, Read};
use colored::Colorize;
use json::JsonValue;
use futures::executor::block_on;
use once_cell::sync::Lazy;

static GITMOJI_URL : &'static str =
    "https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json";
static GITMOJI_CACHE: Lazy<PathBuf> = Lazy::new(|| {
    let cache_folder = ".gitmoji";
    let cache_file = "gitmojis.json";
    let cache_path = home_dir().expect("should have home_dir").join(cache_folder).join(cache_file);
    cache_path
});

#[derive(Debug)]
enum GitmojiError {
    ReqwestError(reqwest::Error),
    JsonError(json::JsonError),
    IOError(std::io::Error),
    Other(String)
}

impl From<reqwest::Error> for GitmojiError {
    fn from(err: reqwest::Error) -> Self {
        GitmojiError::ReqwestError(err)
    }
}

impl From<json::JsonError> for GitmojiError {
    fn from(err: json::JsonError) -> Self {
        GitmojiError::JsonError(err)
    }
}

impl From<std::io::Error> for GitmojiError {
    fn from(err: std::io::Error) -> Self {
        GitmojiError::IOError(err)
    }
}

fn main() {
    let yml = load_yaml!("main.yaml");
    let matches = App::from_yaml(yml)
        .settings(&[ColoredHelp, SubcommandRequired])
        .version(&crate_version!()[..])
        .author(&crate_authors!()[..])
        .set_term_width(80)
        .get_matches();

    if matches.is_present("list") {
        if block_on(list_emojis()).is_err() {
            eprintln!("Could not list gitmojis.");
        }
    }

}

async fn list_emojis() -> Result<(), GitmojiError> {
    fetch_emojis().await?;
    let emojis = get_emojis()?;
    print_emojis(emojis);
    Ok(())
}

async fn fetch_emojis() -> Result<(), GitmojiError> {
    let sp = Spinner::new(Spinners::Dots9, "Fetching the emoji list".into());
    let response: String = reqwest::get(GITMOJI_URL).await?.text().await?;
    create_emoji_cache(json::parse(response.as_str())?)?;
    sp.stop();
    Ok(())
}

fn create_emoji_cache(emojis: JsonValue) -> Result<(), GitmojiError> {
    if !GITMOJI_CACHE.exists() {
        create_dir(GITMOJI_CACHE.parent().expect("should have parent!"))?;
        File::with_options().create(true).write(true).open(GITMOJI_CACHE.clone())?.write_all(emojis.dump().as_bytes())?;
    }
    Ok(())
}

fn get_emojis() -> Result<JsonValue, GitmojiError> {
    let mut string = String::new();
    File::with_options().read(true).open(GITMOJI_CACHE.clone())?.read_to_string(&mut string)?;
    Ok(json::parse(string.as_str())?)
}

fn print_emojis(emojis: JsonValue) {
    if let JsonValue::Object(obj) = emojis {
        for (_key, gitmoji) in obj.iter() {
            println!("{} - {} - {}", gitmoji["emoji"], gitmoji["code"].to_string().blue(), gitmoji["description"]);
        }
    }
}
