use std::io::Result;

mod lib;
use lib::commands::cli;

fn main() -> Result<()> {
    cli()?;
    Ok(())
}
