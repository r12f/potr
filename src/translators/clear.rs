use super::{Translator, TranslatorConfig, TranslatorEngine};
use anyhow::Result;
use async_trait::async_trait;

pub struct ClearTranslator {}

impl ClearTranslator {
    pub fn new(_: TranslatorConfig) -> ClearTranslator {
        ClearTranslator {}
    }
}

#[async_trait]
impl Translator for ClearTranslator {
    fn name(&self) -> TranslatorEngine {
        TranslatorEngine::Clear
    }

    async fn translate(&self, _: &str) -> Result<String> {
        Ok(String::from(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translators::Language;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_clear_translator() {
        let config = TranslatorConfig {
            engine: TranslatorEngine::Clear,
            target_lang: Language::English,
            model: None,
            api_url: None,
            api_key: std::env::var("POTR_API_KEY").unwrap(),
            extra_params: HashMap::new(),
        };
        let translator = ClearTranslator::new(config);
        let result = translator
            .translate("这是一段中文文本。")
            .await
            .expect("Failed to translate");

        assert_eq!(result, "");
    }
}