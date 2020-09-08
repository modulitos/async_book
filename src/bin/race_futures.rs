use futures::future::select_all;
use futures::FutureExt;
use tokio::time::{delay_for, Duration};

async fn get_async_task(task_id: &str, seconds: u64) -> &'_ str {
    println!("starting {}", task_id);
    let duration = Duration::new(seconds, 0);

    delay_for(duration).await;

    println!("{} complete!", task_id);
    task_id
}

#[tokio::main]
async fn main() {
    let futures = vec![

        // `select_all` expects the Futures iterable to implement UnPin, so we use `boxed` here to
        // allocate on the heap:
        // https://users.rust-lang.org/t/the-trait-unpin-is-not-implemented-for-genfuture-error-when-using-join-all/23612/3
        // https://docs.rs/futures/0.3.5/futures/future/trait.FutureExt.html#method.boxed

        get_async_task("task 1", 5).boxed(),
        get_async_task("task 2", 4).boxed(),
        get_async_task("task 3", 1).boxed(),
        get_async_task("task 4", 2).boxed(),
        get_async_task("task 5", 3).boxed(),
    ];

    let (item_resolved, ready_future_index, _remaining_futures) =
        select_all(futures).await;
    println!("item_resolved: {:?}", item_resolved);
    println!("ready_future_index: {:?}", ready_future_index);

    assert_eq!("task 3", item_resolved);
    assert_eq!(2, ready_future_index);
}
