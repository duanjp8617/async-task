use std::thread;
use std::future::Future;
use std::task::{Poll, Context};
use std::pin::{Pin};
use std::time::Duration;

use lazy_static::lazy_static;

use crossbeam::channel::{unbounded, Sender, Receiver};
use crossbeam::sync::{Parker};

use futures::channel::oneshot;

type Task = async_task::Task<i32>;

lazy_static! {
    static ref SENDER : Sender<Task> = {
        let (s, r) : (Sender<Task>, Receiver<Task>) = unbounded();
        for _ in 0..10 {
            let receiver = r.clone();
            thread::spawn(move || {
                receiver.iter().for_each(|t| {
                    t.run();
                });
            });
        }
        s
    };
}

fn block<F, R>(f : F) -> R
where 
    F : Future<Output=R> + Send + 'static,
    R : Send + 'static
{
    // parepare the waker
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = async_task::waker_fn(move || {
        unparker.unpark();
    });

    // create a new task and the associated join handle
    let (task, mut jh) = async_task::spawn(f, |task|{
        SENDER.send(task).unwrap();
    }, 1);

    // schedule the task
    task.schedule();

    // construct a context for polling
    let ctx = &mut Context::from_waker(& waker);

    // keep polling the join handle until the task finishes
    loop {
        match Future::poll(Pin::new(&mut jh), ctx) {
            Poll::Pending => parker.park(),
            Poll::Ready(option_r) => return option_r.unwrap(),
        };
    };
}

fn main() {
    let (s, r) = oneshot::channel();
    let thread_jh = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        s.send(1024).unwrap();
    });

    let result = block(async {
        println!("async block runs");
        r.await.unwrap()
    });

    println!("exepcted 1024: {}", result);
    thread_jh.join().unwrap();
}