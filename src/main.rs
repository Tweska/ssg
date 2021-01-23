mod lib;

use lib::commands::cli;
use std::io::Result;

fn main() -> Result<()> {
    cli()?;
    Ok(())
}
