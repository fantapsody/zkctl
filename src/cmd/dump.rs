use std::error::Error;
use std::{env, fs, io};
use std::path::Path;
use zookeeper::ZooKeeper;
use crate::CMDRunner;
use crate::context::ZKContext;
use clap::Parser;
use regex::Regex;

#[derive(Parser)]
pub struct DumpOpts {
    pub path: String,

    #[arg(long, default_value = "dump")]
    pub dump_path: String,

    #[arg(long)]
    pub value_match: Option<String>,
}

impl CMDRunner for DumpOpts {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>> {
        let zk = zk_opts.client()?;
        let file_path = env::current_dir()?.as_path().join(&self.dump_path);
        fs::create_dir_all(&file_path)?;
        let value_patten = self.value_match.as_ref().map(|vm| {
            regex::Regex::new(vm).unwrap()
        });
        DumpOpts::dump_path(zk.as_ref(), &self.path, &file_path, &value_patten)
    }
}

impl DumpOpts {
    fn dump_path(zk: &ZooKeeper, zk_path: &str, file_path: &Path, value_patten: &Option<Regex>) -> Result<(), Box<dyn Error>> {
        info!("dump {} to {}", zk_path, file_path.to_str().unwrap());
        let (data, stat) = zk.get_data(zk_path, false)?;
        let matched = match value_patten {
            None => true,
            Some(p) => {
                match String::from_utf8(data.clone()) {
                    Ok(s) =>p.is_match(&s),
                    Err(_) => false,
                }
            }
        };
        if matched {
            fs::create_dir_all(file_path)?;
            fs::write(file_path.join("NODE_VALUE"), data)?;
            fs::write(file_path.join("NODE_STAT"), format!("{:?}", stat))?;
        }
        for c in zk.get_children(zk_path, false)? {
            let c_path = file_path.join(&c);
            let c_zk_path = if zk_path == "/" {
                format!("/{}", c)
            } else {
                format!("{}/{}", zk_path, c)
            };
            Self::dump_path(zk, c_zk_path.as_str(), &c_path, value_patten)?;
        }
        Ok(())
    }
}
