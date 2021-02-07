use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;
use std::error::Error;

#[derive(Clap)]
pub struct Set {
    pub path: String,

    pub data: String,

    pub version: Option<i32>,
}

impl CMDRunner for Set {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.client()?;
        let r = zk.set_data(self.path.as_str(), self.data.clone().into_bytes(), self.version)?;
        debug!("{:?}", r);
        Ok(())
    }
}
