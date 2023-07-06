use super::{Translator, TranslatorConfig};
use anyhow::Result;
use async_trait::async_trait;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, ChatCompletionRequest, MessageRole,
};

pub struct OpenAITranslator {
    model: String,
    target_lang: String,
    client: Client,
}

impl OpenAITranslator {
    pub fn new(config: TranslatorConfig) -> OpenAITranslator {
        let client = Client::new(config.api_key.clone());

        OpenAITranslator {
            model: if let Some(model) = config.model {
                model
            } else {
                String::from(openai_api_rs::v1::chat_completion::GPT3_5_TURBO)
            },
            target_lang: config.target_lang,
            client,
        }
    }
}

#[async_trait]
impl Translator for OpenAITranslator {
    async fn translate(&self, text: &str) -> Result<String> {
        let req = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatCompletionMessage {
                    role: MessageRole::system,
                    content: Some(format!("You are a professional translator. Please translate the text into {} without explanation.", self.target_lang)),
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
                    content: Some("The text is: ".to_string() + text),
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
