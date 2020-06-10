use async_std::task;
use std::time::Instant;

#[allow(dead_code)]
pub async fn get_urls() {
    let start = Instant::now();
    let mut tasks = Vec::new();

    for i in 1..41 {
        let run_surf = async move || {
            let url = format!("https://thanks.rust-lang.org/rust/1.{}.0", i);
            surf::get(&url).recv_string().await.unwrap()
        };
        let t = task::spawn(run_surf());
        tasks.push(t);
    };

    for t in tasks {
        t.await;
    }

    println!("{:?}", start.elapsed());
}