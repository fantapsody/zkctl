use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;

#[derive(Clap)]
pub struct List {
    pub path: String,
}

impl CMDRunner for List {
    fn run(&self, zk_opts: &mut ZKContext) -> i32 {
        let zk = zk_opts.zk();
        let rslt = zk.get_children(self.path.as_str(), false);
        match rslt {
            Ok(r) => {
                debug!("{:?}", r);
                r.iter().for_each(|v| {
                    println!("{}", v);
                });
                0
            }
            Err(e) => {
                error!("failed to create [{}]: {}", self.path, e);
                1
            }
        }
    }
}
