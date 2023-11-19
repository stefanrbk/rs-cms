use super::{PluginBase, Transform2Fn};

pub struct ParallelizationPlugin{
    pub base: PluginBase,
    pub max_workers: i32,
    pub worker_flags: u32,
    pub sched: Transform2Fn,
}
