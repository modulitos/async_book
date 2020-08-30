use async_book::timer_future::TimerFuture;
use async_book::executor::{new_executor_and_spawner};
use std::time::Duration;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("running!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(5, 0)).await;
        println!("finished!!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);
    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}
