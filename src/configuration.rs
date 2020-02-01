use crate::GitmojiError;
use crate::prompts::{
    config_for_auto_add,
    config_for_emoji_format,
    config_for_scope_prompt,
    config_for_signed_commit
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmojiFormat {
    CODE,
    EMOJI
}

impl Default for EmojiFormat {
    fn default() -> Self {
        EmojiFormat::CODE
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Configuration {
    auto_add: bool,
    emoji_format: EmojiFormat,
    scope_prompt: bool,
    signed_commit: bool
}

impl Configuration {
    pub fn prompt(&mut self) -> Result<(), GitmojiError> {
        self.auto_add = config_for_auto_add(self.auto_add)?;
        self.emoji_format = config_for_emoji_format(self.emoji_format.clone())?;
        self.scope_prompt = config_for_scope_prompt(self.scope_prompt)?;
        self.signed_commit = config_for_signed_commit(self.signed_commit)?;
        Ok(())
    }
    pub fn load() -> Result<Configuration, GitmojiError> {
        let config: Configuration = confy::load("gitmoji")?;
        Ok(config)
    }

    pub fn store(&self) -> Result<(), GitmojiError> {
        confy::store("gitmoji", self)?;
        Ok(())
    }

    pub fn is_auto_add() -> Result<bool, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.auto_add)
    }

    pub fn emoji_format() -> Result<EmojiFormat, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.emoji_format)
    }

    pub fn is_scope_prompt() -> Result<bool, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.scope_prompt)
    }

    pub fn is_signed_commit() -> Result<bool, GitmojiError> {
        let conf = Self::load()?;
        Ok(conf.signed_commit)
    }
}