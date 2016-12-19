#[macro_use(o, slog_log, slog_info)]
extern crate slog;
extern crate slog_json;
#[macro_use]
extern crate slog_scope;
extern crate slog_stream;
extern crate xcb;
extern crate xdg;

mod core;

use std::ffi::OsString;
use std::env::var_os;
use std::fs::File;
use slog::{Logger, Drain, DrainExt};
use slog_stream::stream;
use slog_scope::set_global_logger;
use xdg::BaseDirectories;

pub fn initialize() {
    let xdg = BaseDirectories::with_prefix("sabiwm").unwrap();
    let path = xdg.place_cache_file("sabiwm.log").unwrap();
    let file = File::create(path).unwrap();
    let file_logger = stream(file, ::slog_json::new().add_default_keys().build());
    let logger = Logger::root(file_logger.ignore_err(),
                              o!("sabiwm" => env!("CARGO_PKG_VERSION")));

    set_global_logger(logger);

    info!("initializing sabiwm");
}
