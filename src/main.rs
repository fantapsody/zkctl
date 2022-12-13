extern crate zookeeper;
extern crate env_logger;

#[macro_use]
extern crate log;
extern crate core;

use crate::opts::parse_opts;
use crate::cmd::runner::CMDRunner;
use std::error::Error;

mod cmd;
mod opts;
mod context;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let opts = parse_opts();
    let mut ctx = opts.create_context();
    match &opts.cmd {
        opts::Command::Get(cmd) => cmd.run(&mut ctx)?,
        opts::Command::Stat(cmd) => cmd.run(&mut ctx)?,
        opts::Command::Ls(cmd) => cmd.run(&mut ctx)?,
        opts::Command::Create(cmd) => cmd.run(&mut ctx)?,
        opts::Command::Set(cmd) => cmd.run(&mut ctx)?,
        opts::Command::Del(cmd) => cmd.run(&mut ctx)?,
        opts::Command::RandWrite(cmd) => cmd.run(&mut ctx)?,
        opts::Command::Dump(cmd) => cmd.run(&mut ctx)?,
    };

    Ok(())
}
