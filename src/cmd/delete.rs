use clap::Clap;
use crate::cmd::runner::CMDRunner;
use zookeeper::ZooKeeperExt;
use crate::context::ZKContext;
use std::error::Error;

#[derive(Clap)]
pub struct DeleteOpts {
    pub path: String,

    #[clap(short = 'r', long)]
    pub recursive: bool,
}

impl CMDRunner for DeleteOpts {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.client()?;
        if self.recursive {
            zk.delete_recursive(self.path.as_str())?
        } else {
            zk.delete(self.path.as_str(), None)?
        };
        Ok(())
    }
}
