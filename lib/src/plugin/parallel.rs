use super::{Plugin, Transform2Fn};

pub struct ParallelizationPlugin {
    pub base: Plugin,
    pub max_workers: i32,
    pub worker_flags: u32,
    pub sched: Transform2Fn,
}
