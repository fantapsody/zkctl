use clap::Clap;
use crate::cmd::runner::CMDRunner;
use zookeeper::ZooKeeperExt;
use crate::context::ZKContext;

#[derive(Clap)]
pub struct Delete {
    pub path: String,

    #[clap(short = 'r', long)]
    pub recursive: bool,
}

impl CMDRunner for Delete {
    fn run(&self, zk_opts: &mut ZKContext) -> i32 {
        let zk = zk_opts.zk();
        let rslt = if self.recursive {
            zk.delete_recursive(self.path.as_str())
        } else {
            zk.delete(self.path.as_str(), None)
        };
        match rslt {
            Ok(r) => {
                debug!("{:?}", r);
                0
            }
            Err(e) => {
                error!("failed to del [{}]: {}", self.path, e);
                1
            }
        }
    }
}
