use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;

#[derive(Clap)]
pub struct Create {
    pub path: String,

    pub data: Option<String>,
}

impl CMDRunner for Create {
    fn run(&self, zk_opts: &mut ZKContext) -> i32 {
        let zk = zk_opts.zk();
        let data = match &self.data {
            Some(d) => d.clone().into_bytes(),
            None => vec![],
        };
        let r = zk.create(self.path.as_str(), data, zookeeper::Acl::open_unsafe().clone(),
                          zookeeper::CreateMode::Persistent);
        match r {
            Ok(rs) => {
                info!("creation of [{}] succeeded: {}", self.path, rs);
                0
            },
            Err(e) => {
                error!("failed to create [{}]: {}", self.path, e);
                1
            }
        }
    }
}
