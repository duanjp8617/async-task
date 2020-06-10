#![feature(async_closure)]
use async_std::task;
use std::time::Instant;

async fn get(url: &str) -> String {
    surf::get(url).recv_string().await.unwrap()
}

fn main() {
    let closure = async || {
        let start = Instant::now();
        let mut tasks = Vec::new();

        for i in 0..40 {
            let url = format!("https://thanks.rust-lang.org/rust/1.{}.0", i);

            tasks.push(task::spawn(async move {
                let html = get(&url).await;

                for line in html.lines() {
                    if line.contains("individuals") {
                        println!("{}", line.trim());
                    }
                }

                println!("{}", &html);
            }));
            
        }

        for t in tasks {
            t.await;
        }

        println!("{:?}", start.elapsed());

    };
    task::block_on(closure());
}
