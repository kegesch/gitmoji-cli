//! Handles everything that concerns the configuration.
use crate::GitmojiError;
use crate::prompts::{config_for_auto_add, config_for_emoji_format, config_for_scope_prompt, config_for_signed_commit, config_for_issue_prompt};

/// Emojiformat which should be used in the commit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmojiFormat {
    /// CODE would like :smile:
    CODE,
    /// EMOJI would be the unicode character itself
    EMOJI
}

impl Default for EmojiFormat {
    fn default() -> Self {
        EmojiFormat::CODE
    }
}

/// Stores the configuration for the cli
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Configuration {
    auto_add: bool,
    emoji_format: EmojiFormat,
    scope_prompt: bool,
    signed_commit: bool,
    reffering_issue: bool,
}

impl Configuration {
    /// Starts the prompts for configuring and uses the current values as defaults.
    pub fn prompt(&mut self) -> Result<(), GitmojiError> {
        self.auto_add = config_for_auto_add(self.auto_add)?;
        self.emoji_format = config_for_emoji_format(self.emoji_format.clone())?;
        self.scope_prompt = config_for_scope_prompt(self.scope_prompt)?;
        self.signed_commit = config_for_signed_commit(self.signed_commit)?;
        self.reffering_issue = config_for_issue_prompt(self.reffering_issue)?;
        Ok(())
    }

    /// loads a configuration from the disk
    pub fn load() -> Result<Configuration, GitmojiError> {
        let config: Configuration = confy::load("gitmoji")?;
        Ok(config)
    }

    /// stores a configuration on the disk
    pub fn store(&self) -> Result<(), GitmojiError> {
        confy::store("gitmoji", self)?;
        Ok(())
    }

    /// is automatic adding enabled
    pub fn is_auto_add() -> Result<bool, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.auto_add)
    }

    /// what is the configured emoji format
    pub fn emoji_format() -> Result<EmojiFormat, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.emoji_format)
    }

    /// is the scope prompt enabled
    pub fn is_scope_prompt() -> Result<bool, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.scope_prompt)
    }

    /// is the signed commit enabled
    pub fn is_signed_commit() -> Result<bool, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.signed_commit)
    }

    /// is the issue prompt enabled
    pub fn is_issue_prompt() -> Result<bool, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.reffering_issue)
    }
}