use super::{Language, Translator, TranslatorConfig, TranslatorEngine};
use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use async_trait::async_trait;

pub struct OpenAITranslator {
    model: String,
    target_lang: Language,
    client: Client<OpenAIConfig>,
}

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
            model: None,
            api_url: None,
            api_key: std::env::var("POTR_TEST_API_KEY_OPENAI").unwrap(),
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
