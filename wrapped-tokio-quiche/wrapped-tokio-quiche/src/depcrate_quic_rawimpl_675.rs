// Generated macro for impl_675 (impl)
macro_rules! Depcrate_quic_rawimpl_675 {
() => {
// Module: crate::quic::raw
// Provides: {"impl_675"}
// Dependencies: {}
impl ConnCloseReceiver { # [doc = " Polls to receive a `connection closed` notification."] pub fn poll_recv (& mut self , cx : & mut Context) -> Poll < () > { loop { let cmd = ready ! (self . 0 . poll_recv (cx)) ; if matches ! (cmd , None | Some (ConnectionMapCommand :: RemoveScid (_))) { return Poll :: Ready (()) ; } } } # [doc = " Waits for a `connection closed` notification."] pub async fn recv (& mut self) { std :: future :: poll_fn (| cx | self . poll_recv (cx)) . await } }
};
}
