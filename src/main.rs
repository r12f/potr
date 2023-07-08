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
    let opts = Opts::parse();
    let log_filter = if opts.verbose {
        "potr=debug"
    } else {
        "potr=info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_filter)).init();
    tracing::debug!("Opts: {:?}", opts);

    let config = opts.to_potr_config();
    let translator_config = opts.to_translator_config();
    tracing::debug!(
        "Config loaded: PotrConfig = {:?}, TranslatorConfig = {:?}",
        config,
        translator_config
    );

    let potr = Potr::new(config, translator_config);

    let cancel_flag = potr.cancel_flag();
    ctrlc::set_handler(move || {
        tracing::info!("Ctrl-C received, stopping processing more messages...");
        cancel_flag.store(true, atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    potr.run().await
}
