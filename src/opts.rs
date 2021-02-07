use clap::Clap;
use crate::cmd::get::Get;
use crate::cmd::create::Create;
use crate::cmd::list::List;
use crate::cmd::set::Set;
use crate::cmd::delete::Delete;
use crate::cmd::stat::Stat;
use crate::cmd::rand_write::RandWrite;
use crate::context::ZKContext;

#[derive(Clap)]
#[clap(version = "1.0", author = "Yang Yang <yyang@streamnative.io>")]
pub struct ZKOpts {
    #[clap(short = 'z', long, default_value = "localhost:2181")]
    pub servers: String,

    #[clap(subcommand)]
    pub cmd: Command,
}

impl ZKOpts {
    pub fn create_context(&self) -> ZKContext {
        return ZKContext::create(self.servers.as_str())
    }
}

#[derive(Clap)]
pub enum Command {
    Get(Get),
    Stat(Stat),
    Ls(List),
    Create(Create),
    Set(Set),
    Del(Delete),

    RandWrite(RandWrite),
}

pub fn parse_opts() -> ZKOpts {
    ZKOpts::parse()
}
