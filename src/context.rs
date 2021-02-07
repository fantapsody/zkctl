use zookeeper::ZooKeeper;
use std::time::Duration;
use std::rc::Rc;
use std::error::Error;

pub struct ZKContext<'a> {
    servers: &'a str,

    client: Option<Rc<ZooKeeper>>,
}

struct ZKWatcher;

impl zookeeper::Watcher for ZKWatcher {
    fn handle(&self, event: zookeeper::WatchedEvent) {
        info!("got event {:?}", event)
    }
}

impl<'a> ZKContext<'a> {
    pub fn create(servers: &str) -> ZKContext {
        return ZKContext {
            servers,
            client: None,
        };
    }

    pub fn client(&mut self) -> Result<Rc<ZooKeeper>, Box<dyn Error>> {
        if self.client.is_none() {
            self.client = Some(Rc::new(ZooKeeper::connect(self.servers, Duration::from_secs(10), ZKWatcher)?))
        }
        Ok(self.client.as_ref().unwrap().clone())
    }
}
