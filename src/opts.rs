use clap::Parser;
use crate::cmd::get::GetOpts;
use crate::cmd::create::CreateOpts;
use crate::cmd::list::ListOpts;
use crate::cmd::set::SetOpts;
use crate::cmd::delete::DeleteOpts;
use crate::cmd::dump::DumpOpts;
use crate::cmd::stat::StatOpts;
use crate::cmd::rand_write::RandWriteOpts;
use crate::context::ZKContext;

#[derive(Parser)]
#[command(version = "1.0", author = "Yang Yang <yyang@streamnative.io>")]
pub struct ZKOpts {
    #[arg(short = 'z', long, default_value = "localhost:2181")]
    pub servers: String,

    #[command(subcommand)]
    pub cmd: Command,
}

impl ZKOpts {
    pub fn create_context(&self) -> ZKContext {
        return ZKContext::create(self.servers.as_str())
    }
}

#[derive(Parser)]
pub enum Command {
    Get(GetOpts),
    Stat(StatOpts),
    Ls(ListOpts),
    Create(CreateOpts),
    Set(SetOpts),
    Del(DeleteOpts),

    RandWrite(RandWriteOpts),
    Dump(DumpOpts),
}

pub fn parse_opts() -> ZKOpts {
    ZKOpts::parse()
}
