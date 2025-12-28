macro_rules! deps {
    () => {
        Running!();
        Cycle!();
    };
}

macro_rules! BlockResult {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) enum BlockResult < 'me > { # [doc = " The query is running on another thread."] Running (Running < 'me >) , # [doc = " Blocking resulted in a cycle."] # [doc = ""] # [doc = " The lock is hold by the current thread or there's another thread that is waiting on the current thread,"] # [doc = " and blocking this thread on the other thread would result in a deadlock/cycle."] Cycle , }
    };
}

BlockResult!()