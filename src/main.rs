#![feature(async_closure)]

mod surf_test;
mod block_on_v1;
mod block_on_v2;
mod block_on_v3;

fn main() {
    block_on_v2::run();
}
