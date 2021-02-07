use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;
use std::error::Error;

#[derive(Clap)]
pub struct List {
    pub path: String,
}

impl CMDRunner for List {
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
