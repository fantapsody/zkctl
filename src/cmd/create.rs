use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;
use std::error::Error;

#[derive(Clap)]
pub struct Create {
    pub path: String,

    pub data: Option<String>,
}

impl CMDRunner for Create {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.zk()?;
        let data = match &self.data {
            Some(d) => d.clone().into_bytes(),
            None => vec![],
        };
        let r = zk.create(self.path.as_str(), data, zookeeper::Acl::open_unsafe().clone(),
                          zookeeper::CreateMode::Persistent)?;
        info!("creation of [{}] succeeded: {}", self.path, r);
        Ok(())
    }
}
