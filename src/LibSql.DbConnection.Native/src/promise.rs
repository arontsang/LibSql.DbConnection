use async_task::Task;
use crate::scheduler::{DotnetScheduler, DotnetSchedulerPointer};

#[repr(C)]
pub struct VoidTask {
    pub(crate) task: Option<Box<Task<()>>>
}

#[no_mangle]
pub extern "C" fn on_completed_void(scheduler: DotnetSchedulerPointer, task: VoidTask, trampoline: extern "C" fn (usize) -> (), callback: usize) {
    if let Some(task) = task.task {
        let scheduler = DotnetScheduler::new(scheduler);
        let task = scheduler.spawn(async move {
            task.await;
            trampoline(callback);
        });
        task.detach();
    }
}