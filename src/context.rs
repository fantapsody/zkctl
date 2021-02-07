use zookeeper::ZooKeeper;
use std::time::Duration;
use std::rc::Rc;
use std::error::Error;

pub struct ZKContext {
    servers: String,

    client: Option<Rc<ZooKeeper>>,
}

struct ZKWatcher;

impl zookeeper::Watcher for ZKWatcher {
    fn handle(&self, event: zookeeper::WatchedEvent) {
        info!("got event {:?}", event)
    }
}

impl ZKContext {
    pub fn create(servers: String) -> ZKContext {
        return ZKContext {
            servers,
            client: None,
        };
    }

    pub fn zk(&mut self) -> Result<Rc<ZooKeeper>, Box<dyn Error>> {
        let client = Rc::new(ZooKeeper::connect(self.servers.as_str(), Duration::from_secs(10), ZKWatcher)?);
        self.client = Some(Rc::clone(&client));
        Ok(client)
    }
}
