#![feature(with_options)]

#[macro_use]
extern crate clap;

use clap::{load_yaml, App, AppSettings::ColoredHelp};
use spinners::{Spinners, Spinner};
use std::path::PathBuf;
use dirs::home_dir;
use std::fs::{create_dir, File};
use std::io::{Write, Read};
use colored::Colorize;
use json::JsonValue;
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
        .settings(&[ColoredHelp])
        .version(&crate_version!()[..])
        .author(&crate_authors!()[..])
        .set_term_width(80)
        .get_matches();

    if matches.is_present("list") {
        if list_emojis(false).is_err() {
            eprintln!("Could not list gitmojis.");
        }
    }
    if matches.is_present("update") {
        if list_emojis(true).is_err() {
            eprintln!("Could not update gitmojis.")
        }
    }

    let query = matches.value_of("search");
    if matches.is_present("search") && query.is_some() {
        if search_emojis(query.unwrap()).is_err() {
            eprintln!("Could not search gitmojis.")
        }
    }

}

fn search_emojis(query: &str) -> Result<(), GitmojiError> {
    let emojis = get_emojis()?;
    let mut filtered = vec![];

    for emo in emojis {
        if emo["name"].to_string().contains(query) || emo["description"].to_string().contains(query) {
            filtered.push(emo.clone())
        }
    }
    print_emojis(filtered);
    Ok(())
}

fn list_emojis(refetch: bool) -> Result<(), GitmojiError> {
    if !GITMOJI_CACHE.exists() || refetch {
        fetch_emojis()?;
    }
    let emojis = get_emojis()?;
    print_emojis(emojis);
    Ok(())
}

fn fetch_emojis() -> Result<(), GitmojiError> {
    let sp = Spinner::new(Spinners::Dots9, "Fetching the emoji list".into());
    let response: String = reqwest::blocking::get(GITMOJI_URL)?.text()?;
    create_emoji_cache(json::parse(response.as_str())?)?;
    sp.stop();
    print!("\r");
    Ok(())
}

fn create_emoji_cache(emojis: JsonValue) -> Result<(), GitmojiError> {
    if !GITMOJI_CACHE.exists() {
        create_dir(GITMOJI_CACHE.parent().expect("should have parent!"))?;
    }
    File::with_options().create(true).write(true).open(GITMOJI_CACHE.clone())?.write_all(emojis.dump().as_bytes())?;
    Ok(())
}

fn get_emojis() -> Result<Vec<JsonValue>, GitmojiError> {
    let mut string = String::new();
    File::with_options().read(true).open(GITMOJI_CACHE.clone())?.read_to_string(&mut string)?;
    let json = json::parse(string.as_str())?;
    if let JsonValue::Array(obj) = json["gitmojis"].clone() {
        return Ok(obj);
    }
    Err(GitmojiError::Other("Could not find gitmoji list in json.".to_owned()))
}

fn print_emojis(emojis: Vec<JsonValue>) {
    for gitmoji in emojis.iter() {
        println!("{} - {} - {}", gitmoji["emoji"], gitmoji["code"].to_string().blue(), gitmoji["description"]);
    }
}