use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[clap(name = "potr", author = "r12f", about = "https://github.com/r12f/potr")]
pub struct Opts {
    #[clap(short, long)]
    pub po_file: String,

    #[clap(short, long, default_value = "en")]
    pub target_lang: String,

    #[clap(short, long, env = "POTR_ENGINE", default_value = "openai")]
    pub engine: String,

    #[clap(short, long, env = "POTR_CONN_API_KEY", default_value = "")]
    pub api_key: String,

    #[clap(short, long, env = "POTR_MODEL")]
    pub model: Option<String>,

    #[clap(short, long = "output", default_value = ".")]
    pub output_dir: String,
}
