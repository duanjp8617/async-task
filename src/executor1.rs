use std::future::Future;
use std::task::{Context, Poll};
use std::pin::{Pin};
use std::marker::Unpin;

use crossbeam::sync::Parker;
use crossbeam::channel::{unbounded, Sender, Receiver};

use futures::channel::oneshot;

lazy_static! {
    static ref SENDER : Sender<async_task::Task<()>> = {
        let (s, r) : (Sender<async_task::Task<()>>, Receiver<async_task::Task<()>>) = unbounded();
        for _ in 0 .. 10 {
            let receiver = r.clone();
            std::thread::spawn(move || {
                receiver.iter().for_each(|t|{t.run();});                
            });
        }
        s
    };
}

fn spawn<F, R>(f : F) -> JoinHandle<R> 
where 
    F : Future<Output = R> + Send + 'static,
    R : Send + 'static
{
    let (task, handle) = async_task::spawn(f, |t| {println!("the task is scheduled"); SENDER.send(t).unwrap();}, ());
    task.schedule();
    
    JoinHandle {
        inner : handle,
    }
}

struct JoinHandle<R> {
    inner : async_task::JoinHandle<R, ()>,
}

impl<R> Unpin for JoinHandle<R> {}

impl<R> Future for JoinHandle<R> {
    type Output = R;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Future::poll(Pin::new(&mut self.get_mut().inner), cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(option_r) => {
                let r = option_r.unwrap();
                Poll::Ready(r)
            }
        }
    }
}


fn block_on<F, R>(f : F) -> R
where
    F : Future<Output = R> + Send + 'static,
    R : Send + 'static 
{   
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = async_task::waker_fn(move || {
        unparker.unpark();
    });
    let ctx = &mut Context::from_waker(&waker);

    let mut jh = spawn(f);
    
    loop {
        match Future::poll(Pin::new(&mut jh), ctx) {
            Poll::Pending => parker.park(),
            Poll::Ready(r) => return r
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    let (s, r) = oneshot::channel();

    let t = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        s.send(1).unwrap();
    });

    let r = block_on(async move {
        let jh = spawn(async move {
            println!("running task");
            r.await.unwrap()
        });

        jh.await
    });

    println!("{}", r);
    t.join().unwrap();
}