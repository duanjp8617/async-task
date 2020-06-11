use std::future::Future;
use std::task::{Poll, Context};
use std::thread;

use futures::channel::oneshot;

fn block_on<F: Future>(f : F) -> F::Output {
    pin_utils::pin_mut!(f);

    let thread = thread::current();
    let waker = async_task::waker_fn(move || {
        println!("thread {:?} unparking thread {:?}", thread::current().id(), thread.id());
        thread.unpark()
    });

    let ctx = &mut Context::from_waker(&waker);

    loop {
        match f.as_mut().poll(ctx) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::park()
        }
    };
}

#[allow(dead_code)]
pub fn run() {
    println!("thread {:?}: run function runs in this thread", thread::current().id());
    let (sender, receiver) = oneshot::channel();

    let t = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(1));
        sender.send(1).unwrap();
    });

    block_on(async move {
        println!("ready to block");
        receiver.await.unwrap()
    });

    t.join().unwrap();
}