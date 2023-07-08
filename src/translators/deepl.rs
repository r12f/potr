use super::{Language, Translator, TranslatorConfig, TranslatorEngine};
use anyhow::Result;
use async_trait::async_trait;
use deepl::{DeepLApi, Lang};

pub struct DeeplTranslator {
    target_lang: Lang,
    client: DeepLApi,
}

impl DeeplTranslator {
    pub fn new(config: TranslatorConfig) -> Result<DeeplTranslator> {
        let client = DeepLApi::with(&config.api_key).new();
        let translator = DeeplTranslator {
            target_lang: Self::langauage_to_deepl_code(config.target_lang)?,
            client,
        };

        Ok(translator)
    }

    fn langauage_to_deepl_code(target_lang: Language) -> Result<Lang> {
        let code = match target_lang {
            Language::Bulgarian => Lang::BG,
            Language::Czech => Lang::CS,
            Language::Danish => Lang::DA,
            Language::German => Lang::DE,
            Language::Greek => Lang::EL,
            Language::English => Lang::EN,
            Language::Spanish => Lang::ES,
            Language::Estonian => Lang::ET,
            Language::Finnish => Lang::FI,
            Language::French => Lang::FR,
            Language::Hungarian => Lang::HU,
            Language::Indonesian => Lang::ID,
            Language::Italian => Lang::IT,
            Language::Japanese => Lang::JA,
            Language::Lithuanian => Lang::LT,
            Language::Latvian => Lang::LV,
            Language::Dutch => Lang::NL,
            Language::Polish => Lang::PL,
            Language::Portuguese => Lang::PT,
            Language::Romanian => Lang::RO,
            Language::Russian => Lang::RU,
            Language::Slovak => Lang::SK,
            Language::Slovenian => Lang::SL,
            Language::Swedish => Lang::SV,
            Language::Turkish => Lang::TR,
            Language::Ukrainian => Lang::UK,
            Language::Chinese => Lang::ZH,
            _ => anyhow::bail!("Unsupported language: {:?}", target_lang),
        };

        Ok(code)
    }
}

#[async_trait]
impl Translator for DeeplTranslator {
    fn name(&self) -> TranslatorEngine {
        TranslatorEngine::DeepL
    }

    async fn translate(&self, text: &str) -> Result<String> {
        let translated = self
            .client
            .translate_text(text, self.target_lang.clone())
            .await?;

        Ok(translated.translations[0].text.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_deepl_translator() {
        let config = TranslatorConfig {
            engine: TranslatorEngine::DeepL,
            target_lang: Language::English,
            api_key: std::env::var("POTR_API_KEY_DEEPL").unwrap(),
            model: None,
            api_base: None,
            api_version: None,
            api_deployment_id: None,
            extra_params: HashMap::new(),
        };
        let translator = DeeplTranslator::new(config).unwrap();
        let result = translator
            .translate("这是一段中文文本。")
            .await
            .expect("Failed to translate");

        assert_eq!(result, "This is a Chinese text.");
    }
}
