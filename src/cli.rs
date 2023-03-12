use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {

    #[arg(short, long)]
    pub(crate) file_path: PathBuf,

    #[arg(short, long)]
    pub(crate) watch: bool,

    #[arg(
        short,
        long,
        default_value_t = tracing::Level::INFO,
        help = "stdout log level (trace, debug, info, warn, error)"
    )]
    pub(crate) log_level: tracing::Level,
}

pub(crate) fn get_args() -> CliArgs {
    CliArgs::parse()
}
