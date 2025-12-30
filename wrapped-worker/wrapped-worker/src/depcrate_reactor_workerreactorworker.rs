// Generated macro for ReactorWorker (struct)
macro_rules! Depcrate_reactor_workerReactorWorker {
() => {
// Module: crate::reactor::worker
// Provides: {"ReactorWorker"}
// Dependencies: {}
pub (crate) struct ReactorWorker < R > where R : 'static + Reactor , { senders : HashMap < HandlerId , UnboundedSender < < R :: Scope as ReactorScoped > :: Input > > , destruct_handle : Option < WorkerDestroyHandle < Self > > , }
};
}
