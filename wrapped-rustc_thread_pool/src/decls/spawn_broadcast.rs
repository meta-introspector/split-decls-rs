macro_rules! deps {
    () => {
        BroadcastContext!();
        Registry!();
    };
}

macro_rules! spawn_broadcast {
    () => {
        deps!();
        # [doc = " Spawns an asynchronous task on every thread in this thread-pool. This task"] # [doc = " will run in the implicit, global scope, which means that it may outlast the"] # [doc = " current stack frame -- therefore, it cannot capture any references onto the"] # [doc = " stack (you will likely need a `move` closure)."] # [doc = ""] # [doc = " For more information, see the [`ThreadPool::spawn_broadcast()`][m] method."] # [doc = ""] # [doc = " [m]: struct.ThreadPool.html#method.spawn_broadcast"] pub fn spawn_broadcast < OP > (op : OP) where OP : Fn (BroadcastContext < '_ >) + Send + Sync + 'static , { unsafe { spawn_broadcast_in (op , & Registry :: current ()) } }
    };
}

spawn_broadcast!();