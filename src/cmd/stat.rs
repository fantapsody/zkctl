use clap::Parser;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;
use std::error::Error;

#[derive(Parser)]
pub struct StatOpts {
    pub path: String,
}

impl StatOpts {
    fn fmt_time(ts: i64) -> String {
        chrono::NaiveDateTime::from_timestamp(ts / 1000, 0).to_string()
    }
}

impl CMDRunner for StatOpts {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.client()?;
        let r = zk.get_data(self.path.as_str(), false)?;
        debug!("{:?}", r);
        println!("cZxid: {:x}", r.1.czxid);
        println!("ctime: {}", Self::fmt_time(r.1.ctime));
        println!("mZxid: {:x}", r.1.mzxid);
        println!("mtime: {}", Self::fmt_time(r.1.mtime));
        println!("pZxid: {:x}", r.1.pzxid);
        println!("cversion: {}", r.1.cversion);
        println!("dataVersion: {}", r.1.version);
        println!("aclVersion: {}", r.1.aversion);
        println!("ephemeralOwner: {}", r.1.ephemeral_owner);
        println!("dataLength: {}", r.1.data_length);
        println!("numChildren: {}", r.1.num_children);
        Ok(())
    }
}
