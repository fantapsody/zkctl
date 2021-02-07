use crate::context::ZKContext;
use std::error::Error;

pub trait CMDRunner {
    fn run(&self, zk_opts: &mut ZKContext) -> Result<(), Box<dyn Error>>;
}
