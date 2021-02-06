extern crate zookeeper;
extern crate env_logger;

#[macro_use]
extern crate log;

use crate::opts::parse_opts;
use crate::cmd::runner::CMDRunner;

mod cmd;
mod opts;
mod context;

fn main() {
    env_logger::init();

    let opts = parse_opts();
    let mut ctx = opts.create_context();
    let ret = match &opts.cmd {
        opts::Command::Get(cmd) => cmd.run(&mut ctx),
        opts::Command::Stat(cmd) => cmd.run(&mut ctx),
        opts::Command::Ls(cmd) => cmd.run(&mut ctx),
        opts::Command::Create(cmd) => cmd.run(&mut ctx),
        opts::Command::Set(cmd) => cmd.run(&mut ctx),
        opts::Command::Del(cmd) => cmd.run(&mut ctx),
        opts::Command::RandWrite(cmd) => cmd.run(&mut ctx),
    };

    std::process::exit(ret);
}
