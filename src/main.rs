extern crate clap;

use crate::app_info::AppInfo;

mod app_info;
mod args_loader;
mod cache_loader;
mod containers;
mod interpreter;
mod operators;

/// qdb -c / --config (path)
fn main() -> std::io::Result<()> {
    cache_loader::exec();
    let app: AppInfo = args_loader::exec();
    app.run();
    Ok(())
}
