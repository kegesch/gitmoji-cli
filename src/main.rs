//! Gitmoji Cli is a command line utility to handle organized commit messages by leveraging gitmojis.
//! It contains an interactive commit interface to handle commit data:
//! * Emoji
//! * Title
//! * Message
//! * Optional Scope
//! * Optional Issue
//! * Optional Signed Commit
//!
//! # Installation
//! With cargo ```cargo install gitmoji-cli```
//! # Usage
//! ## List, Update and Search
//! You can just list all emojis which can be used with ```gitmoji -l```.
//! ```gitmoji -u``` updates the cached emoji list and ```gitmoji -s <query>``` can search
//! for a specific gitmoji.
//! ## Configure
//! You can configure optional prompts or defaults by executing ```gitmoji -g```.
//! ## Commit
//! To start the interactive commit interface type ```gitmoji -c```.
#[macro_use]
extern crate clap;

extern crate confy;

#[macro_use]
extern crate serde_derive;

use clap::{load_yaml, App, AppSettings::ColoredHelp};
use spinners::{Spinners, Spinner};
use std::path::PathBuf;
use dirs::home_dir;
use std::fs::{create_dir, OpenOptions};
use std::io::{Write, Read};
use colored::Colorize;
use json::JsonValue;
use once_cell::sync::Lazy;
use crate::prompts::{Emoji, ask_for_emoji, ask_for_scope, ask_for_title, ask_for_message, ask_for_issue};
use std::process::Command;
use std::str;
use crate::configuration::{Configuration, EmojiFormat};

pub mod prompts;
pub mod configuration;

/// Gitmojis location to fetch from
static GITMOJI_URL : &'static str =
    "https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json";

/// Gitmojis cache folder location
static GITMOJI_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
    let folder_name = ".gitmoji";
    let folder = home_dir().expect("should have home_dir").join(folder_name);
    folder
});

/// Gitmjis chache file location
static GITMOJI_CACHE: Lazy<PathBuf> = Lazy::new(|| {
    let cache_file = "gitmojis.json";
    let cache_path = GITMOJI_FOLDER.join(cache_file);
    cache_path
});

/// Global error collector
#[derive(Debug)]
pub enum GitmojiError {
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

    if matches.is_present("commit") {
        if commit().is_err() {
            eprintln!("Could not commit.");
        }
    }

    if matches.is_present("config") {
        if config().is_err() {
            eprintln!("Could not configure.");
        }
    }

}

/// configures the cli
fn config() -> Result<(), GitmojiError> {
    let mut configuration = Configuration::load()?;
    configuration.prompt()?;
    configuration.store()?;
    Ok(())
}

/// starts the interactive commit interface
fn commit() -> Result<(), GitmojiError> {
    let emojis: Vec<Emoji> = get_emojis()?.iter().map(|val| Emoji::from(val)).collect();
    let emoji = ask_for_emoji(&emojis)?;
    let mut scope = String::new();
    if Configuration::is_scope_prompt()? {
        scope = ask_for_scope()?;
    }
    let title = ask_for_title()?;
    let message = ask_for_message()?;

    let mut commit_title = String::new();
    if Configuration::emoji_format()? == EmojiFormat::CODE {
        commit_title += emoji.clone().code.as_str();
    } else {
        commit_title += emoji.clone().emoji.as_str();
    }
    commit_title += " ";
    if Configuration::is_scope_prompt()? {
        commit_title += scope.as_str();
        commit_title += ": ";
    }
    commit_title += title.as_str();

    if Configuration::is_issue_prompt()? {
        commit_title += " (";
        let issue = ask_for_issue()?;
        commit_title += issue.as_str();
        commit_title += ")";
    }

    if Configuration::is_auto_add()? {
        Command::new("git")
            .arg("add")
            .arg(".")
            .output()?;
    }

    if Configuration::is_signed_commit()? {
        let git_output = Command::new("git")
            .arg("commit")
            .arg("-S")
            .arg("-m")
            .arg(commit_title)
            .arg("-m")
            .arg(message)
            .output()?;

        if git_output.status.success() {
            println!("{}", String::from_utf8_lossy(git_output.stdout.as_ref()));
        } else {
            eprintln!("{}", String::from_utf8_lossy(git_output.stderr.as_ref()));
        }
    } else {
        let git_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_title)
            .arg("-m")
            .arg(message)
            .output()?;

        if git_output.status.success() {
            println!("{}", String::from_utf8_lossy(git_output.stdout.as_ref()));
        } else {
            eprintln!("{}", String::from_utf8_lossy(git_output.stderr.as_ref()));
        }
    }
    Ok(())
}

/// Searches the emojis list by a given `query` and prints the result.
fn search_emojis(query: &str) -> Result<(), GitmojiError> {
    let emojis = get_emojis()?;
    let mut filtered = vec![];

    for emo in emojis {
        if emo["name"].to_string().to_ascii_lowercase().contains(&query.to_ascii_lowercase()) ||
            emo["description"].to_string().to_ascii_lowercase().contains(&query.to_ascii_lowercase()) {
            filtered.push(emo.clone())
        }
    }
    print_emojis(filtered);
    Ok(())
}

/// Lists all cached emojis in the list and prints it.
fn list_emojis(refetch: bool) -> Result<(), GitmojiError> {
    if !GITMOJI_CACHE.exists() || refetch {
        fetch_emojis()?;
    }
    let emojis = get_emojis()?;
    print_emojis(emojis);
    Ok(())
}

/// Fetches the emojis from `GITMOJI_URL` and stores in the cache.
fn fetch_emojis() -> Result<(), GitmojiError> {
    let sp = Spinner::new(Spinners::Dots9, "Fetching the emoji list".into());
    let response: String = reqwest::blocking::get(GITMOJI_URL)?.text()?;
    create_emoji_cache(json::parse(response.as_str())?)?;
    sp.stop();
    print!("\r");
    Ok(())
}

/// Stores `emojis` JsonObject in `GITMOJI_CACHE`
fn create_emoji_cache(emojis: JsonValue) -> Result<(), GitmojiError> {
    if !GITMOJI_CACHE.exists() {
        create_dir(GITMOJI_CACHE.parent().expect("should have parent!"))?;
    }
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(GITMOJI_CACHE.clone())?
        .write_all(emojis.dump().as_bytes())?;
    Ok(())
}

/// Retrieves the emoji list from the cache
fn get_emojis() -> Result<Vec<JsonValue>, GitmojiError> {
    let mut string = String::new();
    OpenOptions::new()
        .read(true)
        .open(GITMOJI_CACHE.clone())?
        .read_to_string(&mut string)?;
    let json = json::parse(string.as_str())?;
    if let JsonValue::Array(obj) = json["gitmojis"].clone() {
        return Ok(obj);
    }
    Err(GitmojiError::Other("Could not find gitmoji list in json.".to_owned()))
}

/// Prints a given list of `emojis` encoded in Json.
fn print_emojis(emojis: Vec<JsonValue>) {
    for gitmoji in emojis.iter() {
        println!("{} - {} - {}", gitmoji["emoji"], gitmoji["code"].to_string().blue(), gitmoji["description"]);
    }
}
