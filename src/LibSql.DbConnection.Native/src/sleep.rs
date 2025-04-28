use std::time::Duration;
use async_io::Timer;
use crate::promise::VoidTask;
use crate::scheduler::{DotnetSchedulerPointer, DotnetScheduler};

#[no_mangle]
pub extern "C" fn dotnet_sleep(scheduler: DotnetSchedulerPointer) -> VoidTask {
    let scheduler = DotnetScheduler::new(scheduler);
    let task = scheduler.spawn(async move {
        println!("Begin sleeping!");
        Timer::after(Duration::from_millis(100)).await;
        println!("Finished sleeping!");
    });

    if task.is_finished(){
        VoidTask { task: None }
    } else {
        VoidTask { task: Some(Box::new(task)) }
    }
}


