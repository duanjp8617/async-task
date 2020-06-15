#![feature(async_closure)]

#[macro_use]
extern crate lazy_static;

mod surf_test;

mod block_on_v1;
mod block_on_v2;
mod block_on_v3;
mod block_on_v4;

mod executor1;

fn main() {
    executor1::run();
}
