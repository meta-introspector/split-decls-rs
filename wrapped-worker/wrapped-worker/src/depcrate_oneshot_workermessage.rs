// Generated macro for Message (enum)
macro_rules! Depcrate_oneshot_workerMessage {
() => {
// Module: crate::oneshot::worker
// Provides: {"Message"}
// Dependencies: {}
pub (crate) enum Message < T > where T : Oneshot , { Finished { handler_id : HandlerId , output : T :: Output , } , }
};
}
