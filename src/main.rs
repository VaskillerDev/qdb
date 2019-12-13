mod args_loader;
mod operators;
use crate::args_loader::{load_operators, load_arguments};


/// qdb (path)
fn main() -> std::io::Result<()>{

    println!("[qdb-core]: load...");
    let args     =  load_arguments();
    let operators = load_operators(&args[1].to_string())?;

    for op in operators {

    }

    println!("[qdb-core]: has been load");

    Ok(())
}