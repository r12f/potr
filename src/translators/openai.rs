use super::{Language, Translator, TranslatorConfig, TranslatorEngine};
use anyhow::Result;
use async_trait::async_trait;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, ChatCompletionRequest, MessageRole,
};

pub struct OpenAITranslator {
    model: String,
    target_lang: Language,
    client: Client,
}

impl OpenAITranslator {
    pub fn new(config: TranslatorConfig) -> OpenAITranslator {
        let client = Client::new(config.api_key.clone());

        OpenAITranslator {
            model: if let Some(model) = config.model {
                model
            } else {
                // By default, we use the GPT3.5 model for cost-saving purpose.
                String::from(openai_api_rs::v1::chat_completion::GPT3_5_TURBO)
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
        let req = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatCompletionMessage {
                    role: MessageRole::system,
                    content: Some(format!("You are a professional translator. Please translate the text into {:?} without explanation.", self.target_lang)),
                    name: None,
                    function_call: None,
                },
                ChatCompletionMessage {
                    role: MessageRole::user,
                    content: Some("I understand. Please give me the text.".to_string()),
                    name: None,
                    function_call: None,
                },
                ChatCompletionMessage {
                    role: MessageRole::user,
                    content: Some(text.to_string()),
                    name: None,
                    function_call: None,
                },
            ],
            functions: None,
            function_call: None,
        };

        let response = self.client.chat_completion(req).await?;
        let result = &response.choices[0].message;
        Ok(result.content.clone().unwrap_or_default())
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
