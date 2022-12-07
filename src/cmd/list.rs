use clap::Parser;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;
use std::error::Error;

#[derive(Parser)]
pub struct ListOpts {
    pub path: String,
}

impl CMDRunner for ListOpts {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.client()?;
        let r = zk.get_children(self.path.as_str(), false)?;
        debug!("{:?}", r);
        r.iter().for_each(|v| {
            println!("{}", v);
        });
        Ok(())
    }
}
