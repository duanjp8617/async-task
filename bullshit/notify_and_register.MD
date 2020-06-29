## Notify and Register

# Notify:

The purpose of notify is to call the stored awaiter. This will wake up the task that await on the join handle of the task.

# Register:

The purpose of register is to attach an awaiter to the task when polling of the join handle resolves to a pending result.

# Timeline of Notify:

1. The NOTIFYING bit of the task state acts as a lock, only one of the notify calls is allowed to enter the critical area. If the NOTIFYING bit is set, then the notify call will do nothing.

2. If the NOTIFYING bit is not set, the notify call will set the NOTIFYING bit and become the only one who enters the critical section. 
    * If the REGISTERING bit is set, then do nothing, as the call in to notify is interleaving with the call into register. In this case, the register call will handle the notification for us.

    * If the REGISTERING bit is not set, we are ready to notify.
        + Take the awaiter out.
        + Clear the NOTIFYING and AWAITER bit.
        + If the awaiter is valid, do one of the followings:
            * If the waker argument is ```None```, directly wake up the awaiter.
            * If the waker argument is ```Some(w)```, and ```w``` is equivalent to the awaiter, then do nothing.
            * If the waker argument is ```Some(w)```, and ```w``` is different to the awaiter, then wake up the awaiter.