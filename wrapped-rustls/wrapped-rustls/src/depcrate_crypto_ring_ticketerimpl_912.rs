// Generated macro for impl_912 (impl)
macro_rules! Depcrate_crypto_ring_ticketerimpl_912 {
() => {
// Module: crate::crypto::ring::ticketer
// Provides: {"impl_912"}
// Dependencies: {}
impl AeadTicketer { # [expect (clippy :: new_ret_no_self)] pub (super) fn new () -> Result < Box < dyn TicketProducer > , Error > { let mut key = [0u8 ; 32] ; SystemRandom :: new () . fill (& mut key) . map_err (| _ | Error :: FailedToGetRandomBytes) ? ; let key = aead :: UnboundKey :: new (TICKETER_AEAD , & key) . unwrap () ; let mut key_name = [0u8 ; 16] ; SystemRandom :: new () . fill (& mut key_name) . map_err (| _ | Error :: FailedToGetRandomBytes) ? ; Ok (Box :: new (Self { alg : TICKETER_AEAD , key : aead :: LessSafeKey :: new (key) , key_name , maximum_ciphertext_len : AtomicUsize :: new (0) , })) } }
};
}
