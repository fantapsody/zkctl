use clap::Clap;
use crate::cmd::runner::CMDRunner;
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::ops::Add;
use zookeeper::{Acl, CreateMode};
use crate::context::ZKContext;
use std::error::Error;

#[derive(Clap)]
pub struct RandWrite {
    pub path: String,

    #[clap(long, default_value = "-1")]
    pub num: i64,

    #[clap(long, default_value = "5")]
    pub sleep_ms: u64,
}

impl CMDRunner for RandWrite {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.client()?;

        let mut i: i64 = 0;
        let mut last_tick = Instant::now();
        while self.num < 0 || i < self.num {
            let value = uuid::Uuid::new_v4().to_string();
            let path = self.path.clone().add("/").add(value.as_str());
            let data = value.into_bytes();
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
