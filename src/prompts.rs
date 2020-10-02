//! Collection of all used prompts
use dialoguer::{Select, Input, Confirm, FuzzySelect};
use json::JsonValue;
use std::io;
use crate::configuration::EmojiFormat;
use dialoguer::theme::{ColorfulTheme, Theme};

/// Struct for emoji data
#[derive(Clone, Debug)]
pub struct Emoji {
    pub code: String,
    description: String,
    pub emoji: String,
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


fn get_theme() -> Box<dyn Theme> {
    Box::from(ColorfulTheme::default())
}

/// Asks what emoji should be used for a commit
pub fn ask_for_emoji(emojis: &[Emoji]) -> Result<&Emoji, io::Error> {
    let theme = get_theme();
    let mut select = FuzzySelect::with_theme(theme.as_ref());
    select.with_prompt("Choose a gitmoji:");
    select.items(emojis);
    select.paged(true);

    let res = select.interact()?;
    let emoji = emojis.iter().find(|emoji| emoji.name == res);
    Ok(emoji.expect("Should be in list"))
}

/// Asks for scope of commit
pub fn ask_for_scope() -> Result<String, io::Error> {
    let theme = get_theme();
    let scope: String = Input::with_theme(theme.as_ref())
        .with_prompt("Enter the scope of current changes:")
        .validate_with(|v: &String| {
            if v.contains('`') {
                Err("Enter a valid scope")
            } else {
                Ok(())
            }
        })
        .interact()?;

    Ok(scope)
}

/// Asks for title of commit
pub fn ask_for_title() -> Result<String, io::Error> {
    let theme = get_theme();
    let mut input = Input::with_theme(theme.as_ref());
    input.with_prompt("Enter the commit title:");
    input.validate_with(|v: &String| {
        if v.contains('`') || v.is_empty() {
            Err("Enter a valid title")
        } else {
            Ok(())
        }
    });

    input.interact()
}

/// Asks for commit message
pub fn ask_for_message() -> Result<String, io::Error> {
    let theme = get_theme();
    let mut input = Input::with_theme(theme.as_ref());
    input.with_prompt("Enter the commit message:");
    input.validate_with(|v: &String| {
        if v.contains('`') {
            Err("Enter a valid message")
        } else {
            Ok(())
        }
    });

    input.interact()
}

/// Asks for referred issue
pub fn ask_for_issue() -> Result<String, io::Error> {
    let theme = get_theme();
    let mut input = Input::with_theme(theme.as_ref());
    input.with_prompt("Enter the referring issue:");
    input.validate_with(|v: &String| {
        if v.contains('`') || v.is_empty() {
            Err("Enter a valid issue")
        } else {
            Ok(())
        }
    });

    input.interact()
}

/// Configure prompt for automatic commit
pub fn config_for_auto_add(default: bool) -> Result<bool, io::Error> {
    let theme = get_theme();
    let mut confirm = Confirm::with_theme(theme.as_ref());
    confirm.with_prompt("Enable automatic \"git add .\":");
    confirm.default(default);

    confirm.interact()
}

/// Configure prompt for signed commits
pub fn config_for_signed_commit(default: bool) -> Result<bool, io::Error> {
    let theme = get_theme();
    let mut confirm = Confirm::with_theme(theme.as_ref());
    confirm.with_prompt("Enable signed commits:");
    confirm.default(default);

    confirm.interact()
}

/// Configure prompt for the scope prompts
pub fn config_for_scope_prompt(default: bool) -> Result<bool, io::Error> {
    let theme = get_theme();
    let mut confirm = Confirm::with_theme(theme.as_ref());
    confirm.with_prompt("Enable scope prompt:");
    confirm.default(default);

    confirm.interact()
}

/// Configure prompt for the issue prompt
pub fn config_for_issue_prompt(default: bool) -> Result<bool, io::Error> {
    let theme = get_theme();
    let mut confirm = Confirm::with_theme(theme.as_ref());
    confirm.with_prompt("Enable referring issue prompt:");
    confirm.default(default);

    confirm.interact()
}

/// Temporary data holder for the emoji selection prompt
struct EmojiFormatSelection {
    emoji_format: EmojiFormat,
    display: String,
}

impl ToString for EmojiFormatSelection {
    fn to_string(&self) -> String {
        self.display.to_string()
    }
}

/// Configure prompt for emoji format
pub fn config_for_emoji_format(default: EmojiFormat) -> Result<EmojiFormat, io::Error> {
    let theme = get_theme();
    let mut select = Select::with_theme(theme.as_ref());
    select.with_prompt("Select how emojis should be used in commits:");
    let code = EmojiFormatSelection {emoji_format: EmojiFormat::CODE, display: ":smile:".to_owned()};
    let emoji = EmojiFormatSelection { emoji_format: EmojiFormat::EMOJI, display: "😄".to_owned()};
    let items = [code, emoji];
    select.items(&items);
    let default_selection = match default {
        EmojiFormat::CODE => 0,
        _ => 1
    };
    select.default(default_selection);

    let selection = select.interact()?;
    let selected_item = items.get(selection).expect("selected item should be in item range");
    match selected_item.emoji_format {
        EmojiFormat::CODE => Ok(EmojiFormat::CODE),
        _ => Ok(EmojiFormat::EMOJI)
    }
}