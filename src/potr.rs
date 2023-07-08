use crate::translators::{self, *};
use anyhow::Result;
use polib::{
    catalog::{Catalog, MessageMutProxy},
    message::{MessageMutView, MessageView},
};
use std::path::Path;

pub struct PotrConfig {
    pub po_file_path: String,
    pub output_file_path: String,
    pub skip_translation: bool,
    pub translate_translated: bool,
    pub translate_code_blocks: bool,
    pub message_limit: i32,
}

pub struct Potr {
    pub config: PotrConfig,
    pub translator_config: TranslatorConfig,
}

impl Potr {
    pub fn new(config: PotrConfig, translator_config: TranslatorConfig) -> Potr {
        Potr {
            config,
            translator_config,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let mut po_file = self.load_po_catelog()?;
        self.translate(&mut po_file).await?;

        tracing::info!("Write to output: {:?}", self.config.output_file_path,);
        polib::po_file::write(&po_file, &Path::new(&self.config.output_file_path))?;

        Ok(())
    }

    fn load_po_catelog(&self) -> Result<Catalog, anyhow::Error> {
        tracing::info!("Loading po file: {}", self.config.po_file_path);
        let po_file = polib::po_file::parse(Path::new(&self.config.po_file_path))?;
        Ok(po_file)
    }

    async fn translate(&self, po_file: &mut Catalog) -> Result<()> {
        let translator = translators::create(self.translator_config.clone());
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
        for mut message in po_file.messages_mut() {
            if self.translate_message(&translator, &mut message).await? {
                translated_count += 1;
            }

            processed_count += 1;
            if processed_count % 10 == 0 {
                tracing::info!(
                    "Processed {} messages, translated {}.",
                    processed_count,
                    translated_count
                );
            }

            if translated_count >= self.config.message_limit {
                tracing::info!("Message limit reached: {}", translated_count);
                break;
            }
        }

        tracing::info!(
            "Translation completed! TotalTranslated = {}",
            translated_count
        );

        Ok(())
    }

    async fn translate_message<'a>(
        &self,
        translator: &Box<dyn Translator>,
        message: &mut MessageMutProxy<'a>,
    ) -> Result<bool> {
        if !self.config.translate_translated && message.is_translated() {
            tracing::debug!("Skip translated message: {}", message.msgid());
            return Ok(false);
        }

        if !self.config.translate_code_blocks && message.msgid().starts_with("```") {
            tracing::debug!("Skip code block message: {}", message.msgid());
            return Ok(false);
        }

        tracing::debug!("Translating message: {}", message.msgid());
        let translated = translator.translate(&message.msgid()).await?;
        tracing::debug!("Translation completed: Result = {}\n", translated);
        message.set_msgstr(translated)?;

        return Ok(true);
    }
}
