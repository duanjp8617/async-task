use std::future::Future;
use std::task::{Poll, Context, Waker};
use std::thread;
use std::cell::RefCell;

use crossbeam::sync::Parker;

use futures::channel::oneshot;

fn block_on<F: Future>(f : F) -> F::Output {
    pin_utils::pin_mut!(f);

    thread_local! {
        static CACHE : RefCell<(Parker, Waker)> = {
            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let waker = async_task::waker_fn(move ||{
                println!("calling waker from thread {:?}", thread::current().id());
                unparker.unpark();
            });
            RefCell::new((parker, waker))
        }
    }

    CACHE.with( |cache| {
        let pair = cache.try_borrow_mut().expect("block_on can only be entered once");

        let ctx = &mut Context::from_waker(& pair.1);

        loop {
            match f.as_mut().poll(ctx) {
                Poll::Ready(output) => return output,
                Poll::Pending => pair.0.park()
            }
        }
    })
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
        let num = receiver.await.unwrap();
        println!("receiving {}", num);
    });

    t.join().unwrap();
}