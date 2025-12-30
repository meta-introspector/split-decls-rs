// Generated macro for OneshotWorker (struct)
macro_rules! Depcrate_oneshot_workerOneshotWorker {
() => {
// Module: crate::oneshot::worker
// Provides: {"OneshotWorker"}
// Dependencies: {}
pub (crate) struct OneshotWorker < T > where T : 'static + Oneshot , { running_tasks : usize , destruct_handle : Option < WorkerDestroyHandle < Self > > , }
};
}
