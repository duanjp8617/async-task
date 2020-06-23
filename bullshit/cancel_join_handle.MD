## Cancel a join handle

# State transition

When calling ```cancel(&self)``` on a join handle, the task may stay in the following three states.

1. ***COMPLTED or CLOSED***: If the task is completed, then you can directly poll the join handle for a ```Some(val)``` result. If the task is closed, then you can also poll the join handle for a ```None``` result. In both cases, canceling the task is meaning less, as the task already stops executing, so we directly quit the cancel function.

2. ***SCHEDULING or RUNNING***: To cancel the task, we need to drop the future stored inside the task. Due to some unknown reason(***why***), the only way to drop the stored future object is to re-schedule the task to run. Therefore, if the task is in scheduling or running state, we just need to mark the state as closed, then we can ensure that, when the task runs again, it will notice that it is canceled and destroy the stored future object. 

    +  Notice that under this state, modifying the state to closed is an important synchronization point. So when running the task, we must make sure that we can synchronize with the state changing to close.

3. *** not SCHEDULED and not RUNNING***: In this case, the task is mostly waiting for an event to call the waker, or idle. We need to mark the state as closed. In particular, after marking the state as closed, we must re-schedule the task, so that it can drop its future object. Any subsequent wake calls by the generated events will not trigger any useful job, as the task has already been canceled.

One thing to notice is that: in state 2 and 3, after the state is marked as closed, if there is an awaiter associated with the task, the awaiter will be notified. However, in both state 2 and state 3, the task will be scheduled to destroy the future object. During this process, the awaiter will also be notified. I think this is some kind of optimization to speed up the notification speed, so that the awaiter does not need to wait for the task finishes scheduling.

# Reference counting.