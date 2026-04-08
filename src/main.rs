mod cli;
mod clone;
mod config;
mod detect;
mod sync;
mod tools;

use anyhow::Result;
use cli::run;

fn main() -> Result<()> {
    run()
}
