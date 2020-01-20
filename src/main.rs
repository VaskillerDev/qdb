extern crate clap;

mod args_loader;
mod cache_loader;
mod containers;
mod operators;

/// qdb -c / --config (path)
fn main() -> std::io::Result<()> {
    cache_loader::exec();
    args_loader::exec();
    Ok(())
}
