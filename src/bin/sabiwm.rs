extern crate sabiwm;
#[macro_use(o, slog_log, slog_error)]
extern crate slog;
#[macro_use]
extern crate slog_scope;

fn main() {
    if let Err(ref e) = ::sabiwm::run() {
        error!("sabiwm stopped: {}", e);
        for e in e.iter().skip(1) {
            error!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            error!("backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}
