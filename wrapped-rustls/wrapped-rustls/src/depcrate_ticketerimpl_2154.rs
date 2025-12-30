// Generated macro for impl_2154 (impl)
macro_rules! Depcrate_ticketerimpl_2154 {
() => {
// Module: crate::ticketer
// Provides: {"impl_2154"}
// Dependencies: {}
impl TicketProducer for TicketRotator { fn encrypt (& self , message : & [u8]) -> Option < Vec < u8 > > { self . maybe_roll (UnixTime :: now ()) ? . current . encrypt (message) } fn decrypt (& self , ciphertext : & [u8]) -> Option < Vec < u8 > > { let state = self . maybe_roll (UnixTime :: now ()) ? ; state . current . decrypt (ciphertext) . or_else (| | { state . previous . as_ref () . and_then (| previous | previous . decrypt (ciphertext)) }) } fn lifetime (& self) -> Duration { self . lifetime } }
};
}
