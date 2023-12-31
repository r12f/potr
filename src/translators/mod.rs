mod clear;
mod clone;
mod deepl;
mod openai;

use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashMap, fmt};
use strum_macros::EnumString;

/// Language list with code.
///
/// This is defined in ISO-639-1: https://localizely.com/iso-639-1-list/.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString)]
pub enum Language {
    #[strum(serialize = "aa")]
    Afar,
    #[strum(serialize = "ab")]
    Abkhazian,
    #[strum(serialize = "ae")]
    Avestan,
    #[strum(serialize = "af")]
    Afrikaans,
    #[strum(serialize = "ak")]
    Akan,
    #[strum(serialize = "am")]
    Amharic,
    #[strum(serialize = "an")]
    Aragonese,
    #[strum(serialize = "ar")]
    Arabic,
    #[strum(serialize = "as")]
    Assamese,
    #[strum(serialize = "av")]
    Avaric,
    #[strum(serialize = "ay")]
    Aymara,
    #[strum(serialize = "az")]
    Azerbaijani,
    #[strum(serialize = "ba")]
    Bashkir,
    #[strum(serialize = "be")]
    Belarusian,
    #[strum(serialize = "bg")]
    Bulgarian,
    #[strum(serialize = "bh")]
    Bihari,
    #[strum(serialize = "bi")]
    Bislama,
    #[strum(serialize = "bm")]
    Bambara,
    #[strum(serialize = "bn")]
    Bengali,
    #[strum(serialize = "bo")]
    Tibetan,
    #[strum(serialize = "br")]
    Breton,
    #[strum(serialize = "bs")]
    Bosnian,
    #[strum(serialize = "ca")]
    Catalan,
    #[strum(serialize = "ce")]
    Chechen,
    #[strum(serialize = "ch")]
    Chamorro,
    #[strum(serialize = "co")]
    Corsican,
    #[strum(serialize = "cr")]
    Cree,
    #[strum(serialize = "cs")]
    Czech,
    #[strum(serialize = "cu")]
    ChurchSlavic,
    #[strum(serialize = "cv")]
    Chuvash,
    #[strum(serialize = "cy")]
    Welsh,
    #[strum(serialize = "da")]
    Danish,
    #[strum(serialize = "de")]
    German,
    #[strum(serialize = "dv")]
    Divehi,
    #[strum(serialize = "dz")]
    Dzongkha,
    #[strum(serialize = "ee")]
    Ewe,
    #[strum(serialize = "el")]
    Greek,
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "eo")]
    Esperanto,
    #[strum(serialize = "es")]
    Spanish,
    #[strum(serialize = "et")]
    Estonian,
    #[strum(serialize = "eu")]
    Basque,
    #[strum(serialize = "fa")]
    Persian,
    #[strum(serialize = "ff")]
    Fulah,
    #[strum(serialize = "fi")]
    Finnish,
    #[strum(serialize = "fj")]
    Fijian,
    #[strum(serialize = "fo")]
    Faroese,
    #[strum(serialize = "fr")]
    French,
    #[strum(serialize = "fy")]
    WesternFrisian,
    #[strum(serialize = "ga")]
    Irish,
    #[strum(serialize = "gd")]
    ScottishGaelic,
    #[strum(serialize = "gl")]
    Galician,
    #[strum(serialize = "gn")]
    Guaraní,
    #[strum(serialize = "gu")]
    Gujarati,
    #[strum(serialize = "gv")]
    Manx,
    #[strum(serialize = "ha")]
    Hausa,
    #[strum(serialize = "he")]
    Hebrew,
    #[strum(serialize = "hi")]
    Hindi,
    #[strum(serialize = "ho")]
    HiriMotu,
    #[strum(serialize = "hr")]
    Croatian,
    #[strum(serialize = "ht")]
    HaitianCreole,
    #[strum(serialize = "hu")]
    Hungarian,
    #[strum(serialize = "hy")]
    Armenian,
    #[strum(serialize = "hz")]
    Herero,
    #[strum(serialize = "ia")]
    Interlingua,
    #[strum(serialize = "id")]
    Indonesian,
    #[strum(serialize = "ie")]
    Interlingue,
    #[strum(serialize = "ig")]
    Igbo,
    #[strum(serialize = "ii")]
    SichuanYi,
    #[strum(serialize = "ik")]
    Inupiaq,
    #[strum(serialize = "io")]
    Ido,
    #[strum(serialize = "is")]
    Icelandic,
    #[strum(serialize = "it")]
    Italian,
    #[strum(serialize = "iu")]
    Inuktitut,
    #[strum(serialize = "ja")]
    Japanese,
    #[strum(serialize = "jv")]
    Javanese,
    #[strum(serialize = "ka")]
    Georgian,
    #[strum(serialize = "kg")]
    Kongo,
    #[strum(serialize = "ki")]
    Kikuyu,
    #[strum(serialize = "kj")]
    Kuanyama,
    #[strum(serialize = "kk")]
    Kazakh,
    #[strum(serialize = "kl")]
    Kalaallisut,
    #[strum(serialize = "km")]
    CentralKhmer,
    #[strum(serialize = "kn")]
    Kannada,
    #[strum(serialize = "ko")]
    Korean,
    #[strum(serialize = "kr")]
    Kanuri,
    #[strum(serialize = "ks")]
    Kashmiri,
    #[strum(serialize = "ku")]
    Kurdish,
    #[strum(serialize = "kv")]
    Komi,
    #[strum(serialize = "kw")]
    Cornish,
    #[strum(serialize = "ky")]
    Kirghiz,
    #[strum(serialize = "la")]
    Latin,
    #[strum(serialize = "lb")]
    Luxembourgish,
    #[strum(serialize = "lg")]
    Ganda,
    #[strum(serialize = "li")]
    Limburgish,
    #[strum(serialize = "ln")]
    Lingala,
    #[strum(serialize = "lo")]
    Lao,
    #[strum(serialize = "lt")]
    Lithuanian,
    #[strum(serialize = "lu")]
    LubaKatanga,
    #[strum(serialize = "lv")]
    Latvian,
    #[strum(serialize = "mg")]
    Malagasy,
    #[strum(serialize = "mh")]
    Marshallese,
    #[strum(serialize = "mi")]
    Maori,
    #[strum(serialize = "mk")]
    Macedonian,
    #[strum(serialize = "ml")]
    Malayalam,
    #[strum(serialize = "mn")]
    Mongolian,
    #[strum(serialize = "mr")]
    Marathi,
    #[strum(serialize = "ms")]
    Malay,
    #[strum(serialize = "mt")]
    Maltese,
    #[strum(serialize = "my")]
    Burmese,
    #[strum(serialize = "na")]
    Nauru,
    #[strum(serialize = "nb")]
    NorwegianBokmal,
    #[strum(serialize = "nd")]
    NorthNdebele,
    #[strum(serialize = "ne")]
    Nepali,
    #[strum(serialize = "ng")]
    Ndonga,
    #[strum(serialize = "nl")]
    Dutch,
    #[strum(serialize = "nn")]
    NorwegianNynorsk,
    #[strum(serialize = "no")]
    Norwegian,
    #[strum(serialize = "nr")]
    SouthNdebele,
    #[strum(serialize = "nv")]
    Navajo,
    #[strum(serialize = "ny")]
    Chichewa,
    #[strum(serialize = "oc")]
    Occitan,
    #[strum(serialize = "oj")]
    Ojibwe,
    #[strum(serialize = "om")]
    Oromo,
    #[strum(serialize = "or")]
    Oriya,
    #[strum(serialize = "os")]
    Ossetian,
    #[strum(serialize = "pa")]
    Panjabi,
    #[strum(serialize = "pi")]
    Pali,
    #[strum(serialize = "pl")]
    Polish,
    #[strum(serialize = "ps")]
    Pushto,
    #[strum(serialize = "pt")]
    Portuguese,
    #[strum(serialize = "qu")]
    Quechua,
    #[strum(serialize = "rm")]
    Romansh,
    #[strum(serialize = "rn")]
    Rundi,
    #[strum(serialize = "ro")]
    Romanian,
    #[strum(serialize = "ru")]
    Russian,
    #[strum(serialize = "rw")]
    Kinyarwanda,
    #[strum(serialize = "sa")]
    Sanskrit,
    #[strum(serialize = "sc")]
    Sardinian,
    #[strum(serialize = "sd")]
    Sindhi,
    #[strum(serialize = "se")]
    NorthernSami,
    #[strum(serialize = "sg")]
    Sango,
    #[strum(serialize = "si")]
    Sinhalese,
    #[strum(serialize = "sk")]
    Slovak,
    #[strum(serialize = "sl")]
    Slovenian,
    #[strum(serialize = "sm")]
    Samoan,
    #[strum(serialize = "sn")]
    Shona,
    #[strum(serialize = "so")]
    Somali,
    #[strum(serialize = "sq")]
    Albanian,
    #[strum(serialize = "sr")]
    Serbian,
    #[strum(serialize = "ss")]
    Swati,
    #[strum(serialize = "st")]
    SothoSouthern,
    #[strum(serialize = "su")]
    Sundanese,
    #[strum(serialize = "sv")]
    Swedish,
    #[strum(serialize = "sw")]
    Swahili,
    #[strum(serialize = "ta")]
    Tamil,
    #[strum(serialize = "te")]
    Telugu,
    #[strum(serialize = "tg")]
    Tajik,
    #[strum(serialize = "th")]
    Thai,
    #[strum(serialize = "ti")]
    Tigrinya,
    #[strum(serialize = "tk")]
    Turkmen,
    #[strum(serialize = "tl")]
    Tagalog,
    #[strum(serialize = "tn")]
    Tswana,
    #[strum(serialize = "to")]
    Tonga,
    #[strum(serialize = "tr")]
    Turkish,
    #[strum(serialize = "ts")]
    Tsonga,
    #[strum(serialize = "tt")]
    Tatar,
    #[strum(serialize = "tw")]
    Twi,
    #[strum(serialize = "ty")]
    Tahitian,
    #[strum(serialize = "ug")]
    Uighur,
    #[strum(serialize = "uk")]
    Ukrainian,
    #[strum(serialize = "ur")]
    Urdu,
    #[strum(serialize = "uz")]
    Uzbek,
    #[strum(serialize = "ve")]
    Venda,
    #[strum(serialize = "vi")]
    Vietnamese,
    #[strum(serialize = "vo")]
    Volapuk,
    #[strum(serialize = "wa")]
    Walloon,
    #[strum(serialize = "wo")]
    Wolof,
    #[strum(serialize = "xh")]
    Xhosa,
    #[strum(serialize = "yi")]
    Yiddish,
    #[strum(serialize = "yo")]
    Yoruba,
    #[strum(serialize = "za")]
    Zhuang,
    #[strum(serialize = "zh")]
    Chinese,
    #[strum(serialize = "zu")]
    Zulu,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString)]
pub enum TranslatorEngine {
    #[strum(serialize = "clear")]
    Clear,
    #[strum(serialize = "clone")]
    Clone,
    #[strum(serialize = "openai")]
    OpenAI,
    #[strum(serialize = "azure-openai")]
    AzureOpenAI,
    #[strum(serialize = "deepl")]
    DeepL,
}

#[derive(Debug, Clone)]
pub struct TranslatorConfig {
    /// Translator engine. E.g. OpenAI, DeepL, etc.
    pub engine: TranslatorEngine,

    /// Target language.
    pub target_lang: Language,

    /// API key.
    pub api_key: String,

    /// Model name. E.g. "gpt-3.5-turbo"
    pub model: Option<String>,

    /// API base URL. E.g. "https://your-resource-name.openai.azure.com"
    pub api_base: Option<String>,

    /// API version. E.g. "2023-03-15-preview"
    pub api_version: Option<String>,

    /// API deployment id.
    pub api_deployment_id: Option<String>,

    /// Extra parameters for each engine.
    pub extra_params: HashMap<String, String>,
}

impl Default for TranslatorConfig {
    fn default() -> Self {
        Self {
            engine: TranslatorEngine::Clone,
            target_lang: Language::English,
            api_key: "".to_string(),
            model: None,
            api_base: None,
            api_version: None,
            api_deployment_id: None,
            extra_params: Default::default(),
        }
    }
}

#[async_trait]
pub trait Translator: Send + Sync {
    fn name(&self) -> TranslatorEngine;

    async fn translate(&self, text: &str) -> Result<String>;
}

pub fn create(config: TranslatorConfig) -> Result<Box<dyn Translator>> {
    let translator: Box<dyn Translator> = match config.engine {
        TranslatorEngine::Clear => Box::new(clear::ClearTranslator::new(config)),
        TranslatorEngine::Clone => Box::new(clone::CloneTranslator::new(config)),
        TranslatorEngine::OpenAI => Box::new(openai::OpenAITranslator::new(config)),
        TranslatorEngine::AzureOpenAI => Box::new(openai::AzureOpenAITranslator::new(config)?),
        TranslatorEngine::DeepL => Box::new(deepl::DeeplTranslator::new(config)?),
    };

    Ok(translator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn languange_can_be_parsed_from_string() {
        let languanges = vec![
            "aa", "ab", "ae", "af", "ak", "am", "an", "ar", "as", "av", "ay", "az", "ba", "be",
            "bg", "bh", "bi", "bm", "bn", "bo", "br", "bs", "ca", "ce", "ch", "co", "cr", "cs",
            "cu", "cv", "cy", "da", "de", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu",
            "fa", "ff", "fi", "fj", "fo", "fr", "fy", "ga", "gd", "gl", "gn", "gu", "gv", "ha",
            "he", "hi", "ho", "hr", "ht", "hu", "hy", "hz", "ia", "id", "ie", "ig", "ii", "ik",
            "io", "is", "it", "iu", "ja", "jv", "ka", "kg", "ki", "kj", "kk", "kl", "km", "kn",
            "ko", "kr", "ks", "ku", "kv", "kw", "ky", "la", "lb", "lg", "li", "ln", "lo", "lt",
            "lu", "lv", "mg", "mh", "mi", "mk", "ml", "mn", "mr", "ms", "mt", "my", "na", "nb",
            "nd", "ne", "ng", "nl", "nn", "no", "nr", "nv", "ny", "oc", "oj", "om", "or", "os",
            "pa", "pi", "pl", "ps", "pt", "qu", "rm", "rn", "ro", "ru", "rw", "sa", "sc", "sd",
            "se", "sg", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr", "ss", "st", "su", "sv",
            "sw", "ta", "te", "tg", "th", "ti", "tk", "tl", "tn", "to", "tr", "ts", "tt", "tw",
            "ty", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "wa", "wo", "xh", "yi", "yo", "za",
            "zh", "zu",
        ];

        for language in languanges {
            Language::from_str(language).unwrap();
        }
    }

    #[test]
    fn translators_can_be_created() {
        let mut config = TranslatorConfig {
            engine: TranslatorEngine::OpenAI,
            target_lang: Language::English,
            api_key: String::from(""),
            model: None,
            api_base: Some("https://your-resource-name.openai.azure.com".to_string()),
            api_version: None,
            api_deployment_id: Some("mock-deployment".to_string()),
            extra_params: HashMap::new(),
        };

        let engines = vec![
            TranslatorEngine::Clear,
            TranslatorEngine::Clone,
            TranslatorEngine::OpenAI,
            TranslatorEngine::AzureOpenAI,
            TranslatorEngine::DeepL,
        ];
        for engine in engines {
            config.engine = engine;
            let translator = create(config.clone()).unwrap();
            assert_eq!(translator.name(), engine);
        }
    }
}
