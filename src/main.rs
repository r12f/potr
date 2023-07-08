mod opts;
mod potr;
mod translators;

use anyhow::Result;
use clap::Parser;
use opts::*;
use potr::*;
use std::sync::atomic;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let opts = Opts::parse();
    let config = opts.to_potr_config();
    let translator_config = opts.to_translator_config();
    let potr = Potr::new(config, translator_config);

    let cancel_flag = potr.cancel_flag();
    ctrlc::set_handler(move || {
        tracing::info!("Ctrl-C received, canceling...");
        cancel_flag.store(true, atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    potr.run().await
}
