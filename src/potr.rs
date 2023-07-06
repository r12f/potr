use crate::translators::{self, *};
use anyhow::Result;
use polib::message::{MessageMutView, MessageView};
use std::path::Path;

pub struct Potr {
    pub po_file_path: String,
    pub output_dir: String,
    pub translator_config: TranslatorConfig,
}

impl Potr {
    pub fn new(
        po_file_path: String,
        output_dir: String,
        translator_config: TranslatorConfig,
    ) -> Potr {
        Potr {
            po_file_path,
            output_dir,
            translator_config,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let translator = translators::create(self.translator_config.clone());

        let mut po_file = polib::po_file::parse(Path::new(&self.po_file_path))?;
        for mut message in po_file.messages_mut() {
            if message.is_translated() {
                continue;
            }

            let translated = translator.translate(&message.msgid()).await?;
            message.set_msgstr(translated)?;
        }

        Ok(())
    }
}
