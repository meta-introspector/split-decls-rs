// Generated macro for TaskKillswitch (struct)
macro_rules! DepcrateTaskKillswitch {
() => {
// Module: crate
// Provides: {"TaskKillswitch"}
// Dependencies: {}
# [doc = " A task killswitch that allows aborting all the tasks spawned with it at"] # [doc = " once. The implementation strives to minimize in-band locking. Spawning a"] # [doc = " future requires a single sharded lock from an internal [`DashMap`]."] # [doc = " Conflicts are expected to be very rare (dashmap defaults to `4 * nproc`"] # [doc = " shards, while each thread can only spawn one task at a time.)"] struct TaskKillswitch { activated : AtomicBool , storage : & 'static ActiveTasks , # [doc = " Watcher that is triggered after all kill signals have been sent (by"] # [doc = " dropping `signal_killed`.) Currently-running tasks are killed after"] # [doc = " their next yield, which may be after this triggers."] all_killed : watch :: Receiver < () > , signal_killed : Mutex < Option < watch :: Sender < () > > > , }
};
}
