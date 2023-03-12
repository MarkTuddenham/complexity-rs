mod cli;
mod languages;
mod extractors;
mod cyclomatic_complexity;
mod watch;
mod utils;

use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, Registry};

use crate::languages::get_language;
use crate::cyclomatic_complexity::parse;
use crate::utils::get_ts_parser;

fn main() {
    let args = cli::get_args();

    let (file_writer, _guard) = dirs::cache_dir()
        .map(|p| p.join("code_complexity"))
        .map(|p| tracing_appender::rolling::daily(p, "log"))
        .map(tracing_appender::non_blocking)
        .unzip();

    let file_logger =
        file_writer.map(|file_writer| fmt::layer().with_ansi(false).with_writer(file_writer));

    let stdout_logger = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(LevelFilter::from_level(args.log_level));

    let subscriber = Registry::default().with(file_logger).with(stdout_logger);

    tracing::subscriber::set_global_default(subscriber).expect("unable to set global tracing subscriber");

    if let Err(e) = app(&args) {
        tracing::error!("{e}");
    }
}
fn app(args: &cli::CliArgs) -> anyhow::Result<()> {

    let language = get_language("rust").unwrap();
    let mut parser = get_ts_parser(language);

    if args.watch {
        watch::watch(&args.file_path, |paths| {
            paths
                .iter()
                .for_each(|path| parse(path, &mut parser, language));
        })
        .unwrap();

        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    } else {
        //TODO: better handle folders
        parse(&args.file_path, &mut parser, language);
    }

    Ok(())
}
