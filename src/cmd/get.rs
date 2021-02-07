use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;
use std::error::Error;

#[derive(Clap)]
pub struct Get {
    pub path: String,
}

impl CMDRunner for Get {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.zk()?;
        let r = zk.get_data(self.path.as_str(), false)?;
        debug!("{:?}", r);
        print!("{}", String::from_utf8(r.0).unwrap());
        Ok(())
    }
}
