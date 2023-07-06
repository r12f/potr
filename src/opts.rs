use clap::Parser;

use crate::translators::*;

#[derive(Debug, Parser)]
#[clap(name = "potr", author = "r12f", about = "https://github.com/r12f/potr")]
pub struct Opts {
    #[clap(short, long)]
    pub po_file: String,

    #[clap(short, long, default_value = "en")]
    pub target_lang: Language,

    #[clap(short, long, env = "POTR_ENGINE", default_value = "openai")]
    pub engine: String,

    #[clap(short, long, env = "POTR_API_KEY", default_value = "")]
    pub api_key: String,

    #[clap(short, long, env = "POTR_MODEL")]
    pub model: Option<String>,

    #[clap(short, long = "output", default_value = ".")]
    pub output_dir: String,
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
}
