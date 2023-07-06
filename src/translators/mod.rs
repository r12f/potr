mod openai;

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TranslatorConfig {
    pub engine: String,
    pub target_lang: String,
    pub api_key: String,
    pub model: Option<String>,
    pub api_url: Option<String>,
    pub extra_params: HashMap<String, String>,
}

impl TranslatorConfig {
    /// Create a new TranslatorConfig from the environment variable "POTR_CONN"
    /// The format of the variable is: "Engine=<ENGINE>;Key=<API_KEY>;Model=<MODEL>;API=<API_URL>;ExtraParam1=foo;ExtraParam2=bar"
    pub fn from_env() -> TranslatorConfig {
        let mut config = TranslatorConfig::default();

        // Parse connection string
        let conn = std::env::var("POTR_CONN").unwrap_or(String::from(""));
        let conn = conn.split(';');
        for param in conn {
            let param = param.split('=').collect::<Vec<&str>>();
            if param.len() != 2 {
                continue;
            }
            match param[0] {
                "Engine" => config.engine = String::from(param[1]),
                "Key" => config.api_key = String::from(param[1]),
                "Model" => config.model = Some(String::from(param[1])),
                "API" => config.api_url = Some(String::from(param[1])),
                _ => {
                    config
                        .extra_params
                        .insert(String::from(param[0]), String::from(param[1]));
                }
            };
        }

        config
    }
}

impl Default for TranslatorConfig {
    fn default() -> Self {
        // By default, we use OpenAI's GPT-3.5 model
        TranslatorConfig {
            engine: String::from("openai"),
            target_lang: String::from("en"),
            model: None,
            api_url: None,
            api_key: String::from(""),
            extra_params: HashMap::new(),
        }
    }
}

#[async_trait]
pub trait Translator: Send + Sync {
    async fn translate(&self, text: &str) -> Result<String>;
}

pub fn create(config: TranslatorConfig) -> impl Translator {
    match config.engine.as_str() {
        "openai" => openai::OpenAITranslator::new(config),
        _ => panic!("Unknown engine: {}", config.engine),
    }
}
