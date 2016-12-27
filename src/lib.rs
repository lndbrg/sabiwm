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
use backend::{Backend, Event};
use std::fs::File;
use slog::{Level, Logger, DrainExt, level_filter};
use slog_stream::stream;
use slog_scope::set_global_logger;
use xdg::BaseDirectories;

pub fn run() -> Result<()> {
    initialize_logger().chain_err(|| "unable to initialize logger")?;

    let xcb = backend::Xcb::new()?;
    let mut workspace: core::Workspace<u32> = core::Workspace::new(0, "Main", None);

    loop {
        match xcb.event() {
            Event::WindowCreated(window) => {
                if !xcb.is_window(window) {
                    continue;
                }
                xcb.resize_window(window, 50, 50);
                workspace = workspace.add(window);
            }
            Event::WindowClosed(window) => {
                workspace = workspace.remove(window);
            }
            //Event::UnknownEvent => {
            //    error!("unknown event");
            //    bail!("unknown event type");
            //}
            _ => ()
        }
    }
}

pub fn initialize_logger() -> Result<()> {
    let xdg =
        BaseDirectories::with_prefix("sabiwm").chain_err(|| "unable to get xdg base directory")?;
    let path = xdg.place_cache_file("sabiwm.log")
        .chain_err(|| "unable to get path for log file")?;
    let file = File::create(path).chain_err(|| "unable to create log file")?;
    let file_logger = stream(file, ::slog_json::new().add_default_keys().build());
    let filter_logger = level_filter(Level::Trace, file_logger);
    let logger = Logger::root(filter_logger.ignore_err(),
                              o!("sabiwm" => env!("CARGO_PKG_VERSION")));

    set_global_logger(logger);

    info!("initialized logger");
    info!("sabiwm version {}", env!("CARGO_PKG_VERSION"));

    Ok(())
}
