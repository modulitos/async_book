use futures::{
    executor::{block_on, ThreadPool, ThreadPoolBuilder},
    future::FutureExt,
    pin_mut, select,
    stream::{FusedStream, Stream, StreamExt},
};
use tokio::runtime::Runtime;
use tokio::task;
use tokio::time::delay_for;

use async_book::executor::new_executor_and_spawner;
use async_book::timer_future::TimerFuture;
use futures::task::{LocalSpawnExt, SpawnExt};
use std::thread::Thread;
use std::time::Duration;

async fn race_tasks() {
    async fn task_1() {
        println!("starting task 1");
        let duration = Duration::new(5, 0);

        // This only works for tokio runtime:
        // delay_for(duration).await;

        // TimerFuture allows us to race both tasks at once because internally it's spawning
        // thread::sleep in parallel:

        TimerFuture::new(duration).await;

        // This blocks the entire executor, since it blocks the main thread:
        // task::block_in_place(|| {
        //     std::thread::sleep(duration);
        // });

        println!("task 1 complete!");
    }

    async fn task_2() {
        println!("starting task 2");
        let duration = Duration::new(1, 0);

        // delay_for(duration).await;

        TimerFuture::new(duration).await;

        // std::thread::sleep(duration);
        // task::block_in_place(|| {
        //     std::thread::sleep(duration);
        // });

        println!("task 2 complete!");
    }

    // // the FusedFuture trait is required because select must not poll a future after it has
    // // completed:
    //
    let t1 = task_1().fuse();
    let t2 = task_2().fuse();

    // Unpin is necessary because the futures used by select are not taken by value, but by mutable
    // reference. By not taking ownership of the future, uncompleted futures can be used again after
    // the call to select

    pin_mut!(t1, t2);

    let res = select! {
      () = t1 => {
      println!("task 1 completed first.");
      "task1"
      },
      () = t2 => {
      println!("task 2 completed first.");
      "task2"
      }
    };
    assert_eq!(res, "task2");
}

async fn count() {
    let mut a_fut = futures::future::ready(4);
    let mut b_fut = futures::future::ready(6);
    let mut total = 0;

    // a_fut or b_fut will have completed the second time through the loop. Because the future
    // returned by future::ready implements FusedFuture, it's able to tell select not to poll it
    // again.

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => unreachable!(), // never runs (futures are ready, then complete)
        };
    }
    assert_eq!(total, 10);
}

// Note that streams have a corresponding FusedStream trait. Streams which implement this trait or
// have been wrapped using .fuse() will yield FusedFuture futures from their .next() / .try_next()
// combinators.

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}

// TODO: This doesn't quite work correctly, since it waits for ALL of the tasks to complete, even though
// we're only supposed to be selecting the first completed task, then exiting.

fn custom_executor() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("spawning!");
        race_tasks().await;
        // count().await;
        println!("finished executing.");
    });

    drop(spawner);

    executor.run();
}

// This is a multi-threaded executor.

fn thread_pool_executor() {
    let mut pool = ThreadPoolBuilder::new().pool_size(4).create().unwrap();
    let handle = pool
        .spawn_with_handle(async {
            println!("starting inside!");
            race_tasks().await;
            println!("finished executing");
        })
        .unwrap();
    block_on(handle);
}

fn tokio_runtime() {
    // Create the runtime
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        tokio::spawn(async move {
            race_tasks().await;
        })
        .await
        .unwrap();
    });
}

fn main() {
    // custom_executor();
    thread_pool_executor();
    // tokio_runtime();
}
