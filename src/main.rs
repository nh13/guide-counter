pub mod commands;
pub mod guide;
pub mod io;

use anyhow::Result;
use clap::Parser;
use commands::command::Command;
use commands::*;
use enum_dispatch::enum_dispatch;
use env_logger::Env;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[enum_dispatch(Command)]
#[derive(Parser, Debug)]
enum Subcommand {
    Count(count::Count),
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Args = Args::parse();
    args.subcommand.execute()
}
