macro_rules! deps {
    () => {
        Latch!();
    };
}

macro_rules! LockLatch {
    () => {
        deps!();
        # [doc = " A Latch starts as false and eventually becomes true. You can block"] # [doc = " until it becomes true."] # [derive (Debug)] pub (super) struct LockLatch { m : Mutex < bool > , v : Condvar , }
    };
}

LockLatch!();