use super::{Language, Translator, TranslatorConfig, TranslatorEngine};
use anyhow::Result;
use async_openai::{
    config::{AzureConfig, Config, OpenAIConfig},
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use async_trait::async_trait;

pub struct OpenAITranslatorT<C: Config> {
    model: String,
    target_lang: Language,
    client: Client<C>,
}

impl<C: Config> OpenAITranslatorT<C> {
    async fn do_translate(&self, text: &str) -> Result<String> {
        let req = CreateChatCompletionRequestArgs::default()
            .model(self.model.clone())
            .messages([
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::System)
                    .content(format!("You are a professional translator. Please translate the text into {:?} without explanation.", self.target_lang))
                    .build()?,
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content("I understand. Please give me the text.".to_string())
                    .build()?,
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(text.to_string())
                    .build()?,
            ]).build()?;

        let response = self.client.chat().create(req).await?;
        let result = match &response.choices[0].message.content {
            Some(content) => content.clone(),
            None => String::from(""),
        };
        Ok(result)
    }
}

pub type OpenAITranslator = OpenAITranslatorT<OpenAIConfig>;

impl OpenAITranslator {
    pub fn new(config: TranslatorConfig) -> OpenAITranslator {
        let openai_config = OpenAIConfig::new().with_api_key(config.api_key.clone());
        let client = Client::with_config(openai_config);

        OpenAITranslator {
            model: if let Some(model) = config.model {
                model
            } else {
                // By default, we use the GPT3.5 model for cost-saving purpose.
                String::from("gpt-3.5-turbo")
            },
            target_lang: config.target_lang,
            client,
        }
    }
}

#[async_trait]
impl Translator for OpenAITranslator {
    fn name(&self) -> TranslatorEngine {
        TranslatorEngine::OpenAI
    }

    async fn translate(&self, text: &str) -> Result<String> {
        self.do_translate(text).await
    }
}

pub type AzureOpenAITranslator = OpenAITranslatorT<AzureConfig>;

impl AzureOpenAITranslator {
    pub fn new(config: TranslatorConfig) -> Result<AzureOpenAITranslator> {
        if config.api_base.is_none() {
            anyhow::bail!("Azure OpenAI requires an API URL");
        }

        if config.api_deployment_id.is_none() {
            anyhow::bail!("Azure OpenAI requires an deployment id");
        }

        let api_version = if config.api_version.is_none() {
            String::from("2023-03-15-preview")
        } else {
            config.api_version.unwrap()
        };

        let openai_config = AzureConfig::new()
            .with_api_key(config.api_key.clone())
            .with_api_base(config.api_base.as_ref().unwrap().clone())
            .with_api_version(api_version)
            .with_deployment_id(config.api_deployment_id.as_ref().unwrap().clone());

        let client = Client::with_config(openai_config);

        let translator = OpenAITranslatorT {
            model: if let Some(model) = config.model {
                model
            } else {
                // By default, we use the GPT3.5 model for cost-saving purpose.
                String::from("gpt-3.5-turbo")
            },
            target_lang: config.target_lang,
            client,
        };

        Ok(translator)
    }
}

#[async_trait]
impl Translator for AzureOpenAITranslator {
    fn name(&self) -> TranslatorEngine {
        TranslatorEngine::AzureOpenAI
    }

    async fn translate(&self, text: &str) -> Result<String> {
        self.do_translate(text).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_openai_translator() {
        let config = TranslatorConfig {
            engine: TranslatorEngine::OpenAI,
            target_lang: Language::English,
            api_key: std::env::var("POTR_TEST_API_KEY_OPENAI").unwrap(),
            model: None,
            api_base: None,
            api_version: None,
            api_deployment_id: None,
            extra_params: HashMap::new(),
        };
        let translator = OpenAITranslator::new(config);
        let result = translator
            .translate("这是一段中文文本。")
            .await
            .expect("Failed to translate");

        assert_eq!(result, "This is a Chinese text.");
    }

    #[tokio::test]
    async fn test_azure_openai_translator() {
        // Still waiting in the waitlist ...
        if std::env::var("POTR_API_KEY_AZURE_OPENAI").is_err() {
            return;
        }

        let config = TranslatorConfig {
            engine: TranslatorEngine::OpenAI,
            target_lang: Language::English,
            api_key: std::env::var("POTR_API_KEY_AZURE_OPENAI").unwrap(),
            model: None,
            api_base: Some(std::env::var("POTR_API_BASE_AZURE_OPENAI").unwrap()),
            api_version: None,
            api_deployment_id: Some(std::env::var("POTR_API_DEPLOYMENT_ID_AZURE_OPENAI").unwrap()),
            extra_params: HashMap::new(),
        };
        let translator = OpenAITranslator::new(config);
        let result = translator
            .translate("这是一段中文文本。")
            .await
            .expect("Failed to translate");

        assert_eq!(result, "This is a Chinese text.");
    }
}
