use std::future::Future;
use std::ptr::NonNull;
use async_task::{Runnable, ScheduleInfo};
pub type DotnetSchedulerPointer = extern "C" fn (PortableRunnable, u32) -> ();

#[no_mangle]
pub extern "C" fn run_runnable(runnable: PortableRunnable) {
    let runnable: Runnable = runnable.into();
    runnable.run();
}

#[repr(C)]
pub struct PortableRunnable {
    runnable: NonNull<()>
}


impl From<Runnable> for PortableRunnable {
    fn from(runnable: Runnable) -> Self {
        let runnable = runnable.into_raw();
        PortableRunnable { runnable }
    }
}

impl Into<Runnable> for PortableRunnable {
    fn into(self) -> Runnable {
        unsafe { Runnable::from_raw(self.runnable) }
    }
}

pub struct DotnetScheduler {
    scheduler: extern "C" fn (PortableRunnable, u32) -> ()
}

impl DotnetScheduler {
    pub fn new(scheduler: DotnetSchedulerPointer) -> DotnetScheduler {
        DotnetScheduler { scheduler }
    }
}

impl DotnetScheduler {
    pub fn spawn<F>(self: &Self, future: F) -> async_task::Task<F::Output>
    where F: Future + Send + 'static, F::Output: Send + 'static {

        let scheduler = self.scheduler.clone();
        let schedule = move |runnable: Runnable, info: ScheduleInfo| {
            let is_wake = if info.woken_while_running { 1 } else { 0 };
            scheduler(PortableRunnable::from(runnable), is_wake);
        };

        let (runnable, task) = async_task::spawn(future, async_task::WithInfo(schedule));
        runnable.schedule();
        task
    }
}

