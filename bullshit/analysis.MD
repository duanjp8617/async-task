# Overview

This is intended as a complete analysis of async-task crate. I can't find any resources that explain the internals of async-task. And the documentation embeded in the async-task source code is not designed for new-comers to this crate.

This analysis is written for someon like me, who is new to Rust's amazing async eco-system, and would like to peek into the internals of how async-task actually works.

THe major difficutly in understanding async-task is to undertstand the concurrency mechanism. Async-task is a lock-free data structure, it extensively uses atomic variables to synchronize states among multiple threads. Therefore, before reading this analysis, one is expected to study some basic knowledge about C++20 memory model (Rust adopts the exact same memory model).

By reading this analysis and studying the source code of async-task crate, I hope that one can understand how async-task works, and learn how to design lock-free data structures.

# Source Code Structure

Async-task has three important modules:

1. raw.rs: 


