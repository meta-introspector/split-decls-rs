// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl TicketProducer for Ticketer { fn encrypt (& self , plain : & [u8]) -> Option < Vec < u8 > > { Some (plain . to_vec ()) } fn decrypt (& self , cipher : & [u8]) -> Option < Vec < u8 > > { Some (cipher . to_vec ()) } fn lifetime (& self) -> Duration { Duration :: from_secs (60 * 60 * 6) } }
};
}
