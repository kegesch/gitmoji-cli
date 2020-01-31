use dialoguer::{Select, Input};
use enquirer::ColoredTheme;
use json::JsonValue;
use std::io;

#[derive(Clone, Debug)]
pub struct Emoji {
    pub code: String,
    description: String,
    emoji: String,
    name: String
}

impl ToString for Emoji {
    fn to_string(&self) -> String {
        format!("{} - {}", self.emoji, self.description)
    }
}

impl From<&JsonValue> for Emoji {
    fn from(val: &JsonValue) -> Self {
        Emoji {
            code: val["code"].to_string(),
            description: val["description"].to_string(),
            emoji: val["emoji"].to_string(),
            name: val["name"].to_string()
        }
    }
}

pub fn ask_for_emoji(emojis: &Vec<Emoji>) -> Result<&Emoji, io::Error> {
    let theme = ColoredTheme::default();
    let mut select = Select::with_theme(&theme);
    select.with_prompt("Choose a gitmoji:");
    select.items(emojis);
    select.paged(true);
    select.default(0);

    // TODO autocomplete

    let res = select.interact()?;
    Ok(emojis.get(res).expect("Should be in list"))
}

pub fn ask_for_scope() -> Result<String, io::Error> {
    let theme = ColoredTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt("Enter the scope of current changes:");

    // TODO validate

    input.interact()
}

pub fn ask_for_title() -> Result<String, io::Error> {
    let theme = ColoredTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt("Enter the commit title:");

    // TODO validate

    input.interact()
}

pub fn ask_for_message() -> Result<String, io::Error> {
    let theme = ColoredTheme::default();
    let mut input = Input::with_theme(&theme);
    input.with_prompt("Enter the commit message:");

    // TODO validate

    input.interact()
}