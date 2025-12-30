// Generated macro for make_acquire_future (function)
macro_rules! Depcrate_sync_mpscmake_acquire_future {
() => {
// Module: crate::sync::mpsc
// Provides: {"make_acquire_future"}
// Dependencies: {}
async fn make_acquire_future < T > (data : Option < Sender < T > > ,) -> Result < OwnedPermit < T > , PollSendError < T > > { match data { Some (sender) => sender . reserve_owned () . await . map_err (| _ | PollSendError (None)) , None => unreachable ! ("this future should not be pollable in this state") , } }
};
}
