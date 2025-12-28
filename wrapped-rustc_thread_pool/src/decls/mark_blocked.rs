macro_rules! deps {
    () => {
        WorkerThread!();
    };
}

macro_rules! mark_blocked {
    () => {
        deps!();
        # [doc = " Mark a Rayon worker thread as blocked. This triggers the deadlock handler"] # [doc = " if no other worker thread is active"] # [inline] pub fn mark_blocked () { let worker_thread = WorkerThread :: current () ; assert ! (! worker_thread . is_null ()) ; unsafe { let registry = & (* worker_thread) . registry ; registry . sleep . mark_blocked (& registry . deadlock_handler) } }
    };
}

mark_blocked!();