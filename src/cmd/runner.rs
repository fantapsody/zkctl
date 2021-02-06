use crate::context::ZKContext;

pub trait CMDRunner {
    fn run(&self, zk_opts: &mut ZKContext) -> i32;
}
