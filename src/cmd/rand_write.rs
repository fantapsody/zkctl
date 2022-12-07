use clap::Parser;
use crate::cmd::runner::CMDRunner;
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::ops::Add;
use zookeeper::{Acl, CreateMode};
use crate::context::ZKContext;
use std::error::Error;
use rand::RngCore;

#[derive(Parser)]
pub struct RandWriteOpts {
    pub path: String,

    #[clap(long, default_value = "-1")]
    pub num: i64,

    #[clap(long, default_value = "5")]
    pub sleep_ms: u64,

    #[clap(long, default_value = "1000")]
    pub node_size: u64,
}

impl CMDRunner for RandWriteOpts {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.client()?;

        let mut i: i64 = 0;
        let mut last_tick = Instant::now();
        while self.num < 0 || i < self.num {
            let value = uuid::Uuid::new_v4().to_string();
            let path = self.path.clone().add("/").add(value.as_str());

            let mut data: Vec<u8> = Vec::with_capacity(self.node_size as usize);
            for _j in 0 .. self.node_size {
                data.push((('a' as u32) + (rand::thread_rng().next_u32()) % 26) as u8)
            }
            let r = zk.create(path.as_str(), data, Acl::open_unsafe().clone(), CreateMode::Persistent);
            match r {
                Ok(rs) => debug!("creation of {} succeeded: {}", path, rs),
                Err(e) => error!("failed to create {}: {}", path, e)
            }
            if self.sleep_ms > 0 {
                sleep(Duration::from_millis(self.sleep_ms));
            }
            i += 1;
            let now = Instant::now();
            if now - last_tick > Duration::from_secs(5) {
                info!("written [{}] records, rps: [{}]", i, i / 5);
                i = 0;
                last_tick = now;
            }
        }
        Ok(())
    }
}
