use clap::Parser;

use crate::{potr::PotrConfig, translators::*};

#[derive(Debug, Parser)]
#[clap(name = "potr", author = "r12f", about = "https://github.com/r12f/potr")]
pub struct Opts {
    #[clap(short, long = "po")]
    pub po_file_path: String,

    /// Target languange. Please use the short code defined in ISO-639-1.
    #[clap(short, long, default_value = "en")]
    pub target_lang: Language,

    #[clap(short, long, env = "POTR_ENGINE", default_value = "openai")]
    pub engine: TranslatorEngine,

    #[clap(short, long, env = "POTR_API_KEY", default_value = "")]
    pub api_key: String,

    #[clap(short, long, env = "POTR_MODEL")]
    pub model: Option<String>,

    #[clap(short, long = "output")]
    pub output_file_path: Option<String>,

    /// Skip translation, only generate po file.
    #[clap(long, visible_alias = "st")]
    pub skip_translation: bool,

    /// Process translated message. By default, translated messages are skipped.
    #[clap(long, visible_alias = "pt")]
    pub process_translated: bool,

    /// Skip regular text messages. By default, text messages are translated.
    #[clap(long)]
    pub skip_text: bool,

    /// Process code blocks (starts with ```). By default, code blocks are skipped.
    #[clap(long, visible_alias = "pc")]
    pub process_code_blocks: bool,

    /// Limit the number of messages to translate.
    #[clap(short, long, default_value = "0")]
    pub limit: i32,

    /// Print verbose logs.
    #[clap(short, long)]
    pub verbose: bool,
}

impl Opts {
    pub fn to_translator_config(&self) -> TranslatorConfig {
        TranslatorConfig {
            engine: self.engine.clone(),
            target_lang: self.target_lang,
            model: self.model.clone(),
            api_url: None,
            api_key: self.api_key.clone(),
            extra_params: Default::default(),
        }
    }

    pub fn to_potr_config(&self) -> PotrConfig {
        let output_file_path = match &self.output_file_path {
            Some(path) => path.clone(),
            None => self.po_file_path.clone(),
        };

        PotrConfig {
            po_file_path: self.po_file_path.clone(),
            output_file_path: output_file_path,
            skip_translation: self.skip_translation,
            skip_translated: !self.process_translated,
            skip_code_blocks: !self.process_code_blocks,
            skip_text: self.skip_text,
            message_limit: self.limit,
        }
    }
}
