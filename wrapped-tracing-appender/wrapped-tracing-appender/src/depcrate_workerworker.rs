// Generated macro for Worker (struct)
macro_rules! Depcrate_workerWorker {
() => {
// Module: crate::worker
// Provides: {"Worker"}
// Dependencies: {}
pub (crate) struct Worker < T : Write + Send + 'static > { writer : T , receiver : Receiver < Msg > , shutdown : Receiver < () > , }
};
}
