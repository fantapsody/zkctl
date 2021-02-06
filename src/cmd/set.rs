use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;

#[derive(Clap)]
pub struct Set {
    pub path: String,

    pub data: String,

    pub version: Option<i32>
}

impl CMDRunner for Set {
    fn run(&self, zk_opts: &mut ZKContext) -> i32 {
        let zk = zk_opts.zk();
        let rslt = zk.set_data(self.path.as_str(), self.data.clone().into_bytes(), self.version);
        match rslt {
            Ok(r) => {
                debug!("{:?}", r);
                0
            }
            Err(e) =>{
                error!("failed to update [{}] to [{}]: {}", self.path, self.data, e);
                1
            }
        }
    }
}
