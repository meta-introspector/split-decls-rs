// Generated macro for impl_527 (impl)
macro_rules! Depcrate_quic_connectionimpl_527 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_527"}
// Dependencies: {}
impl HandshakeInfo { pub (crate) fn new (start_time : Instant , timeout : Option < Duration >) -> Self { Self { start_time , timeout , time_handshake : None , } } # [doc = " The time at which the connection was created."] # [inline] pub fn start_time (& self) -> Instant { self . start_time } # [doc = " How long the handshake took to complete."] # [inline] pub fn elapsed (& self) -> Duration { self . time_handshake . unwrap_or_default () } pub (crate) fn set_elapsed (& mut self) { let elapsed = self . start_time . elapsed () ; self . time_handshake = Some (elapsed) } pub (crate) fn deadline (& self) -> Option < Instant > { self . timeout . map (| timeout | self . start_time + timeout) } pub (crate) fn is_expired (& self) -> bool { self . timeout . is_some_and (| timeout | self . start_time . elapsed () >= timeout) } }
};
}
