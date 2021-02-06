use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;

#[derive(Clap)]
pub struct Get {
    pub path: String,
}

impl CMDRunner for Get {
    fn run(&self, zk_opts: &mut ZKContext) -> i32 {
        let zk = zk_opts.zk();
        let rslt = zk.get_data(self.path.as_str(), false);
        match rslt {
            Ok(r) => {
                debug!("{:?}", r);
                print!("{}", String::from_utf8(r.0).unwrap());
                0
            }
            Err(e) =>{
                error!("failed to create [{}]: {}", self.path, e);
                1
            }
        }
    }
}
