use crate::translators::{self, *};
use anyhow::Result;
use polib::message::{MessageMutView, MessageView};
use std::path::Path;

pub struct Potr {
    pub po_file_path: String,
    pub output_file_path: String,
    pub translator_config: TranslatorConfig,
}

impl Potr {
    pub fn new(
        po_file_path: String,
        output_file_path: Option<String>,
        translator_config: TranslatorConfig,
    ) -> Potr {
        let output_file_path = output_file_path.unwrap_or_else(|| po_file_path.clone());

        Potr {
            po_file_path,
            output_file_path,
            translator_config,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let translator = translators::create(self.translator_config.clone());

        tracing::info!("Loading po file: {}", self.po_file_path);
        let mut po_file = polib::po_file::parse(Path::new(&self.po_file_path))?;

        tracing::info!(
            "Starting tranlating po file: TotalMessageCount = {}",
            po_file.count()
        );

        let mut translated_count = 0;
        for mut message in po_file.messages_mut() {
            if message.is_translated() {
                continue;
            }

            let translated = translator.translate(&message.msgid()).await?;
            message.set_msgstr(translated)?;

            translated_count += 1;
            if translated_count % 10 == 0 {
                tracing::info!("Translated {} messages", translated_count);
            }
        }

        tracing::info!(
            "Translation completed! Write to output: {:?}, TotalTranslated = {}",
            self.output_file_path,
            translated_count
        );
        polib::po_file::write(&po_file, &Path::new(&self.output_file_path))?;

        Ok(())
    }
}
