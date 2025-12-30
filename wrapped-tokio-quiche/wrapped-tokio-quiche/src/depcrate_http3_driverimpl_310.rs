// Generated macro for impl_310 (impl)
macro_rules! Depcrate_http3_driverimpl_310 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_310"}
// Dependencies: {}
impl < H : DriverHooks > H3Controller < H > { # [doc = " Gets a mut reference to the [`H3Event`] receiver for the paired"] # [doc = " [H3Driver]."] pub fn event_receiver_mut (& mut self) -> & mut UnboundedReceiver < H :: Event > { self . h3_event_recv . as_mut () . expect ("No event receiver on H3Controller") } # [doc = " Takes the [`H3Event`] receiver for the paired [H3Driver]."] pub fn take_event_receiver (& mut self) -> UnboundedReceiver < H :: Event > { self . h3_event_recv . take () . expect ("No event receiver on H3Controller") } # [doc = " Creates a [`QuicCommand`] sender for the paired [H3Driver]."] pub fn cmd_sender (& self) -> RequestSender < H :: Command , QuicCommand > { RequestSender { sender : self . cmd_sender . clone () , _r : Default :: default () , } } # [doc = " Sends a GOAWAY frame to initiate a graceful connection shutdown."] pub fn send_goaway (& self) { let _ = self . cmd_sender . send (H3Command :: GoAway . into ()) ; } }
};
}
