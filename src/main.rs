#![feature(async_closure)]

mod surf_test;
mod block_on;

fn main() {
    block_on::run();
}
