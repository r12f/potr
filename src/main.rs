mod opts;
mod potr;
mod translators;

use anyhow::Result;
use clap::Parser;
use opts::*;
use potr::*;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let translator_config = opts.to_translator_config();
    let potr = Potr::new(opts.po_file, opts.output_dir, translator_config);
    potr.run().await
}
