use crate::translators::{self, *};
use anyhow::Result;
use polib::{
    catalog::{Catalog, MessageMutProxy},
    message::{MessageMutView, MessageView},
};
use regex::Regex;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct PotrConfig {
    pub po_file_path: String,
    pub output_file_path: String,
    pub skip_translation: bool,
    pub skip_translated: bool,
    pub skip_code_blocks: bool,
    pub skip_text: bool,
    pub source_regex: Option<Regex>,
    pub message_limit: i32,
}

impl Default for PotrConfig {
    fn default() -> Self {
        Self {
            po_file_path: Default::default(),
            output_file_path: Default::default(),
            skip_translation: false,
            skip_translated: true,
            skip_code_blocks: true,
            skip_text: false,
            source_regex: None,
            message_limit: 0,
        }
    }
}

pub struct Potr {
    pub config: PotrConfig,
    pub translator_config: TranslatorConfig,
    pub is_canceled: Arc<AtomicBool>,

    source_parser_regex: Regex,
}

impl Potr {
    pub fn new(config: PotrConfig, translator_config: TranslatorConfig) -> Potr {
        Potr {
            config,
            translator_config,
            is_canceled: Arc::new(AtomicBool::new(false)),
            source_parser_regex: Regex::new(r"[^:]+:\d+").unwrap(),
        }
    }

    pub fn cancel_flag(&self) -> Arc<AtomicBool> {
        self.is_canceled.clone()
    }

    pub async fn run(&self) -> Result<()> {
        let mut po_file = self.load_po_catelog()?;
        self.translate(&mut po_file).await?;
        self.write_output_file(po_file)?;
        Ok(())
    }

    fn load_po_catelog(&self) -> Result<Catalog, anyhow::Error> {
        tracing::info!("Loading po file: {}", self.config.po_file_path);
        let po_file = polib::po_file::parse(Path::new(&self.config.po_file_path))?;
        Ok(po_file)
    }

    async fn translate(&self, po_file: &mut Catalog) -> Result<()> {
        let translator = translators::create(self.translator_config.clone())?;
        if self.config.skip_translation {
            tracing::info!(
                "Traslation skipped: TotalMessageCount = {}",
                po_file.count()
            );
            return Ok(());
        }

        tracing::info!(
            "Starting tranlating po file: TotalMessageCount = {}",
            po_file.count()
        );

        let mut processed_count = 0;
        let mut translated_count = 0;
        let mut failed_count = 0;
        for mut message in po_file.messages_mut() {
            if self.is_canceled.load(Ordering::SeqCst) {
                break;
            }

            match self.translate_message(&translator, &mut message).await {
                Ok(translated) => {
                    if translated {
                        translated_count += 1;
                    }
                }
                Err(e) => {
                    failed_count += 1;

                    tracing::error!(
                        "Failed to translate message: Error = {}, Message = {}",
                        e,
                        message.msgid()
                    );
                }
            }

            processed_count += 1;
            if processed_count % 10 == 0 {
                tracing::info!(
                    "Processed {} messages, translated {}, failed {}.",
                    processed_count,
                    translated_count,
                    failed_count
                );
            }

            if self.config.message_limit > 0 && translated_count >= self.config.message_limit {
                tracing::info!("Message limit reached: {}", translated_count);
                break;
            }
        }

        tracing::info!(
            "Translation completed! Processed = {}, TotalTranslated = {}, TotalFailed = {}",
            processed_count,
            translated_count,
            failed_count
        );

        Ok(())
    }

    async fn translate_message<'a>(
        &self,
        translator: &Box<dyn Translator>,
        message: &mut MessageMutProxy<'a>,
    ) -> Result<bool> {
        if self.config.skip_translated && message.is_translated() {
            tracing::debug!("Skip translated message: {}", message.msgid());
            return Ok(false);
        }

        if message.msgid().starts_with("```") {
            if self.config.skip_code_blocks {
                tracing::debug!("Skip code block message: {}", message.msgid());
                return Ok(false);
            }
        } else if self.config.skip_text {
            tracing::debug!("Skip regular text message: {}", message.msgid());
            return Ok(false);
        }

        if let Some(source_regex) = &self.config.source_regex {
            let message_source_str = message.source();
            let message_sources: Vec<&str> = self
                .source_parser_regex
                .find_iter(message_source_str)
                .map(|m| m.as_str().trim())
                .collect();

            if !message_sources.iter().any(|s| source_regex.is_match(s)) {
                tracing::debug!(
                    "Skip message not matching source regex: {}, Source = {}",
                    message.msgid(),
                    message_source_str
                );
                return Ok(false);
            }
        }

        tracing::debug!("Translating message: {}", message.msgid());
        let translated = translator.translate(&message.msgid()).await?;
        tracing::debug!("Translation completed: Result = {}\n", translated);
        message.set_msgstr(translated)?;

        return Ok(true);
    }

    fn write_output_file(&self, po_file: Catalog) -> Result<(), anyhow::Error> {
        tracing::info!("Write to output: {:?}", self.config.output_file_path,);
        polib::po_file::write(&po_file, &Path::new(&self.config.output_file_path))?;
        Ok(())
    }
}
