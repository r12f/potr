use super::{Translator, TranslatorConfig, TranslatorEngine};
use anyhow::Result;
use async_trait::async_trait;

pub struct CloneTranslator {}

impl CloneTranslator {
    pub fn new(_: TranslatorConfig) -> CloneTranslator {
        CloneTranslator {}
    }
}

#[async_trait]
impl Translator for CloneTranslator {
    fn name(&self) -> TranslatorEngine {
        TranslatorEngine::Clone
    }

    async fn translate(&self, text: &str) -> Result<String> {
        Ok(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translators::Language;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_clone_translator() {
        let config = TranslatorConfig {
            engine: TranslatorEngine::Clone,
            target_lang: Language::English,
            model: None,
            api_url: None,
            api_key: std::env::var("POTR_API_KEY").unwrap(),
            extra_params: HashMap::new(),
        };
        let translator = CloneTranslator::new(config);
        let result = translator
            .translate("这是一段中文文本。")
            .await
            .expect("Failed to translate");

        assert_eq!(result, "这是一段中文文本。");
    }
}
