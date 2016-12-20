#[macro_use(o, slog_log, slog_debug, slog_trace, slog_warn, slog_error, slog_info)]
extern crate slog;
extern crate slog_json;
#[macro_use]
extern crate slog_scope;
extern crate slog_stream;
extern crate xcb;
extern crate xdg;
#[macro_use]
extern crate error_chain;

pub mod backend;
pub mod core;

mod errors {
    error_chain!{}
}

use errors::*;
use std::fs::File;
use slog::{Logger, DrainExt};
use slog_stream::stream;
use slog_scope::set_global_logger;
use xdg::BaseDirectories;

pub fn run() -> Result<()> {
    initialize_logger().chain_err(|| "unable to initialize logger")?;

    let stack = core::Stack::from(42u32);
    let vec: Vec<_> = stack.add(17).integrate();

    Ok(())
}

pub fn initialize_logger() -> Result<()> {
    let xdg =
        BaseDirectories::with_prefix("sabiwm").chain_err(|| "unable to get xdg base directory")?;
    let path = xdg.place_cache_file("sabiwm.log")
        .chain_err(|| "unable to get path for log file")?;
    let file = File::create(path).chain_err(|| "unable to create log file")?;
    let file_logger = stream(file, ::slog_json::new().add_default_keys().build());
    let logger = Logger::root(file_logger.ignore_err(),
                              o!("sabiwm" => env!("CARGO_PKG_VERSION")));

    set_global_logger(logger);

    info!("initialized logger");
    info!("sabiwm version {}", env!("CARGO_PKG_VERSION"));

    Ok(())
}
