# Overview

This is intended as a complete analysis of async-task crate. I can't find any resources that explain the internals of async-task. And the documentation embeded in the async-task source code is not designed for new-comers to this crate.

This analysis is written for someon like me, who is new to Rust's amazing async eco-system, and would like to peek into the internals of how async-task actually works.

THe major difficutly in understanding async-task is to undertstand the concurrency mechanism. Async-task is a lock-free data structure, it extensively uses atomic variables to synchronize states among multiple threads. Therefore, before reading this analysis, one is expected to study some basic knowledge about C++20 memory model (Rust adopts the exact same memory model).

By reading this analysis and studying the source code of async-task crate, I hope that one can understand how async-task works, and learn how to design lock-free data structures.

# Source Code Structure

Async-task has three important modules:

1. raw.rs: Contain the definition and method implementation of ```RawTask```. The ```Task``` is just a wrapper around ```RawTask```. 

2. headers: Contain the definition of the task header, which stores important state imformation used throughout the lifetime of the task.

3. join_handle.rs: Contain the definition and method implementation of join_handle. In fact, ```JoinHandle``` is just a wrapper around ```RawTask``` as well. 


# Major Abastractions.

1. Task:
    + The ```Task``` object is only generated under three conditions:
        * When a new task and a join handle are first created with spawn.
        * Inside the schedule function.
        * The schedule function usually sends the task to a queue, therefore you can also obtain the task by polling from this queue.

    + ```schedule``` function. The ```Task``` object has a public schedule function. Calling the schedule function will consume the task object and sends the task object into the user-defined schedule closure. The ```schedule``` function can be called in the following situations:
        * After the task is created with ```spawn```, creator has ownership of the task object, and can call the ```schedule``` function.
        * After the task is obtained by polling the queue, the poller should run the task, but the poller can also choose to schedule the task by calling ```schedule``` again, though this is absolutely not recommended.
        * Inside the user-defined schedule closure, the task object is also available, and the closure can call the ```schedule``` function. However, this will immediately cause a stack overflow.
    
    + The public ```schedule``` method of ```Task``` is actually implemented with the ```schedule``` function of the ```RawTask```. Except for being called under the previous three situations, the private ```schedule``` function of ```RawTask``` will also be called under the following situations:
        * A waker object can schedule a raw-task by calling the ```schedule``` function when the awaited event happens.
        * When the waker is dropped, it will decrease the reference counter, if the reference drops to zero and the join handle is dropped as well, then the drop function will 
        * When the join handle actively cancels the task, it will schedule the task again to drop the future object if the task is not yet completed.
        * When the join handle is dropped, it will schedule the task again to drop the future object if the task is not yet completed.

    
    + With the ```Task``` object, you can do the following things:
        * ```task.schedule()```: The ```schedule``` function consumes the task, and then call the user-defined schedule closure. Inside the schedule closure, the task is usually sent to some queues.
        * ```task.run()```: The ```run``` function runs the task, and is the major method that drives the state of the task forward. ```run``` method is usually called under the following two conditions

2. Waker:
    + The ```Waker``` object is created under the following conditions:
        * In ```RawTask::schedule```, a temporary waker object will be created before calling the schedule closure. This only happens if the schedule closure has captured some variables (***Why***)
        * The ```Task``` object exposes a public API to obtain a waker associated with the underlying ```RawTask```. Therefore, whenever you can access ```Task``` object, you can call this API to obtain an associated waker.
        * The ```JoinHandle``` object exposes a public API to obtain an associated waker object with the underyling ```RawTask```
        * When polling the task, the async closure may create new wakers if the task is not completed and returns pending. 
    
    + A waker is used to re-schedule the task in case that some asynchronous event happens. When the event happens, the owner of the waker will call ```waker.wake()``` or ```waker.wake_by_ref()```. 

    + 

3. JoinHandle:

