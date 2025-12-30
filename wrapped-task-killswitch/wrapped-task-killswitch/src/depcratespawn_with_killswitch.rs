// Generated macro for spawn_with_killswitch (function)
macro_rules! Depcratespawn_with_killswitch {
() => {
// Module: crate
// Provides: {"spawn_with_killswitch"}
// Dependencies: {}
# [doc = " Spawns a new asynchronous task and registers it in the crate's global"] # [doc = " killswitch."] # [doc = ""] # [doc = " Under the hood, [`tokio::spawn`] schedules the actual execution."] # [inline] pub fn spawn_with_killswitch (fut : impl Future < Output = () > + Send + 'static ,) -> Option < Id > { TASK_KILLSWITCH . spawn_task (fut) }
};
}
