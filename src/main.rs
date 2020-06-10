#![feature(async_closure)]
use async_std::task;

mod surf_test;

fn main() {
    task::block_on(surf_test::get_urls());
}
