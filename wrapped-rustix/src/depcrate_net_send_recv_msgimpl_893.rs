// Generated macro for impl_893 (impl)
macro_rules! Depcrate_net_send_recv_msgimpl_893 {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"impl_893"}
// Dependencies: {}
impl < 'slice , 'fd > Extend < SendAncillaryMessage < 'slice , 'fd > > for SendAncillaryBuffer < '_ , 'slice , 'fd > { fn extend < T : IntoIterator < Item = SendAncillaryMessage < 'slice , 'fd > > > (& mut self , iter : T) { iter . into_iter () . all (| msg | self . push (msg)) ; } }
};
}
