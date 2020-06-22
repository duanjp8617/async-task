use std::future::Future;
use std::task::{Context, Poll};
use std::pin::{Pin};
use std::marker::Unpin;

use crossbeam::sync::Parker;
use crossbeam::channel::{unbounded, Sender, Receiver};

use futures::channel::oneshot;
use lazy_static::lazy_static;

type Task = async_task::Task<i32>;

lazy_static! {
    static ref SENDER : Sender<Task> = {
        let (s, r) : (Sender<Task>, Receiver<Task>) = unbounded();
        for _ in 0 .. 10 {
            let receiver = r.clone();
            std::thread::spawn(move || {
                receiver.iter().for_each(|t|{
                    async_task::tprint(&format!("run task {}", t.tag()));
                    t.run();                    
                });                
            });
        }
        s
    };
}

fn spawn<F, R>(f : F, t : i32) -> JoinHandle<R, i32> 
where 
    F : Future<Output = R> + Send + 'static,
    R : Send + 'static,
{
    let (task, handle) = async_task::spawn(f, |task| {        
        async_task::tprint(&format!("[Task {}] [schedule closure] -> schedule closure is called", task.tag()));
        SENDER.send(task).unwrap();
    }, t);
    async_task::tprint(&format!("[Task {}] [executor1::spawn] -> creating a new task", task.tag()));
    task.schedule();
    
    JoinHandle {
        inner : handle,
    }
}

struct JoinHandle<R, T> {
    inner : async_task::JoinHandle<R, T>,
}

impl<R, T> Unpin for JoinHandle<R, T> {}

impl<R, T> Future for JoinHandle<R, T> {
    type Output = R;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            async_task::tprint(&format!("[Task {}] [Self-defined JoinHandle::poll] -> polling Self-defined JoinHandle", &*(self.inner.tag() as *const T as *const i32)));
        }
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
        async_task::tprint("[Waker] -> waker is called to unpark");
        unparker.unpark();
    });
    let ctx = &mut Context::from_waker(&waker);

    let mut jh = spawn(f, 1);
    loop {
        match Future::poll(Pin::new(&mut jh), ctx) {
            Poll::Pending => parker.park(),
            Poll::Ready(r) => return r
        }
    }
}


fn main() {
    let (s, r) = oneshot::channel();

    let t = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));        
        async_task::tprint("[Thread Closure] -> send to the sender channel");
        s.send(1).unwrap();
    });

    let r = block_on(async move {
        let jh = spawn(async move {            
            async_task::tprint("[async closure] -> run async_closure");
            r.await.unwrap()
        }, 2);

        jh.await
    });

    println!("{}", r);
    t.join().unwrap();
}