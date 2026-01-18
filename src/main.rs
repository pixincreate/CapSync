mod cli;
mod config;
mod detect;
mod sync;

use anyhow::Result;
use cli::run;

fn main() -> Result<()> {
    run()
}
