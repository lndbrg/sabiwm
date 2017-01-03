//! # SabiWM - 錆WM (Rust WM)
//!
//! SabiWM is a window manager written entirely in safe Rust.
//! It aims to be a reliable, highly customizable tiling window manager
//! in the spirit of XMonad, I3, bspwm and the like.
//!
//! ## SabiWM Lib
//!
//! The core part of SabiWM is its library. It serves as a
//! learning basis for people who want to write their own window manager
//! or as an easy place to customize it.
//!
//! The library itself is split into several parts, shortly explained below:
//!
//! ### Core
//!
//! The [`Core`] module
//!
//! The core module contains all the necessary data structures to handle the internal
//! state to tile windows, manage focus, screens, workspaces, etc.
//!
//! ### Backend
//!
//! The backend module contains the general backend trait to abstract
//! away from all the different backends, e.g. XCB, Wayland, Redox and all the others out there.
//!
//! ### Config
//!
//! The config module will contain several ways to configure the WM. All of them shall
//! be interchangeable. The first and easiest way is a simple TOML file and on top of that
//! a ctl daemon, a LUA interface and direct IPC.
//!
//! [`Core`]: core/index.html
//! ['Backend']: backend/index.html

#![deny(missing_docs)]

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

#[macro_use]
mod macros;
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

/// Run the actual window manager
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
            // Event::UnknownEvent => {
            //    error!("unknown event");
            //    bail!("unknown event type");
            // }
            _ => (),
        }
    }
}

/// Initialize the logger
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
