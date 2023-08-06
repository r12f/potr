use anyhow::Result;
use clap::Parser;
use potr::*;
use regex::Regex;

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

    #[clap(short = 'k', long)]
    pub api_key: Option<String>,

    // API base. Used by Azure OpenAI, e.g. "https://your-resource-name.openai.azure.com".
    #[clap(long, env = "POTR_API_BASE_AZURE_OPENAI")]
    pub api_base: Option<String>,

    // API version. Used by Azure OpenAI, e.g. "2023-03-15-preview".
    #[clap(long, env = "POTR_API_VERSION_AZURE_OPENAI")]
    pub api_version: Option<String>,

    // API deployment id. Used by Azure OpenAI.
    #[clap(long, env = "POTR_API_DEPLOYMENT_ID_AZURE_OPENAI")]
    pub api_deployment_id: Option<String>,

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

    /// Source file path regex to translate. By default, all files are translated.
    #[clap(long)]
    pub source: Option<String>,

    /// Regex to include messages to translate. By default, all messages are translated.
    #[clap(long = "include")]
    pub include_message: Option<String>,

    /// Regex to exclude messages to translate. By default, all messages are translated.
    #[clap(long = "include")]
    pub exclude_message: Option<String>,

    /// Print verbose logs.
    #[clap(short, long)]
    pub verbose: bool,
}

impl Opts {
    pub fn to_translator_config(&self) -> Result<TranslatorConfig> {
        let api_key = match &self.api_key {
            Some(key) => key.clone(),
            None => match self.engine {
                TranslatorEngine::OpenAI => match std::env::var("POTR_API_KEY_OPENAI") {
                    Ok(key) => key,
                    Err(_) => {
                        anyhow::bail!(
                            "OpenAI API key is not specified, please specify it via \"-k\" option or POTR_API_KEY_OPENAI environment variable."
                        );
                    }
                },
                TranslatorEngine::AzureOpenAI => match std::env::var("POTR_API_KEY_AZURE_OPENAI") {
                    Ok(key) => key,
                    Err(_) => {
                        anyhow::bail!(
                            "Azure OpenAI service API key is not specified, please specify it via \"-k\" option or POTR_API_KEY_AZURE_OPENAI environment variable."
                        );
                    }
                },
                TranslatorEngine::DeepL => match std::env::var("POTR_API_KEY_DEEPL") {
                    Ok(key) => key,
                    Err(_) => {
                        anyhow::bail!(
                            "DeepL API key is not specified, please specify it via \"-k\" option or POTR_API_KEY_DEEPL environment variable."
                        );
                    }
                },
                _ => "".to_string(),
            },
        };

        let config = TranslatorConfig {
            engine: self.engine.clone(),
            target_lang: self.target_lang,
            model: self.model.clone(),
            api_base: self.api_base.clone(),
            api_key,
            api_version: self.api_version.clone(),
            api_deployment_id: self.api_deployment_id.clone(),
            extra_params: Default::default(),
        };

        Ok(config)
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
            source_regex: match &self.source {
                Some(s) => Some(Regex::new(s).unwrap()),
                None => None,
            },
            include_message_regex: match &self.include_message {
                Some(s) => Some(Regex::new(s).unwrap()),
                None => None,
            },
            exclude_message_regex: match &self.exclude_message {
                Some(s) => Some(Regex::new(s).unwrap()),
                None => None,
            },
            message_limit: self.limit,
        }
    }
}
