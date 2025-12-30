// Generated macro for impl_995 (impl)
macro_rules! Depcrate_crypto_ringimpl_995 {
() => {
// Module: crate::crypto::ring
// Provides: {"impl_995"}
// Dependencies: {}
impl TicketerFactory for Ring { # [doc = " Make the recommended `Ticketer`."] # [doc = ""] # [doc = " This produces tickets:"] # [doc = ""] # [doc = " - where each lasts for at least 6 hours,"] # [doc = " - with randomly generated keys, and"] # [doc = " - where keys are rotated every 6 hours."] # [doc = ""] # [doc = " The encryption mechanism used is Chacha20Poly1305."] fn ticketer (& self) -> Result < Arc < dyn TicketProducer > , Error > { # [cfg (feature = "std")] { Ok (Arc :: new (TicketRotator :: new (TicketRotator :: SIX_HOURS , AeadTicketer :: new ,) ?)) } # [cfg (not (feature = "std"))] { Err (Error :: General ("Ring::ticketer() relies on std-only RwLock via TicketRotator" . into () ,)) } } fn fips (& self) -> bool { fips () } }
};
}
