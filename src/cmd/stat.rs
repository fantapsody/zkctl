use clap::Clap;
use crate::cmd::runner::{CMDRunner};
use crate::context::ZKContext;

#[derive(Clap)]
pub struct Stat {
    pub path: String,
}

impl Stat {
    fn fmt_time(ts: i64) -> String {
        chrono::NaiveDateTime::from_timestamp(ts / 1000, 0).to_string()
    }
}

impl CMDRunner for Stat {
    fn run(&self, zk_opts: &mut ZKContext) -> i32 {
        let zk = zk_opts.zk();
        let rslt = zk.get_data(self.path.as_str(), false);
        match rslt {
            Ok(r) => {
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
                0
            }
            Err(e) => {
                error!("failed to create [{}]: {}", self.path, e);
                1
            }
        }
    }
}
