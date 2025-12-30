// Generated macro for tests (module)
macro_rules! Depcrate_crypto_ring_ticketertests {
() => {
// Module: crate::crypto::ring::ticketer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: crypto :: TicketerFactory ; use crate :: crypto :: ring :: Ring ; # [test] fn basic_pairwise_test () { let t = Ring . ticketer () . unwrap () ; let cipher = t . encrypt (b"hello world") . unwrap () ; let plain = t . decrypt (& cipher) . unwrap () ; assert_eq ! (plain , b"hello world") ; } # [test] fn refuses_decrypt_before_encrypt () { let t = Ring . ticketer () . unwrap () ; assert_eq ! (t . decrypt (b"hello") , None) ; } # [test] fn refuses_decrypt_larger_than_largest_encryption () { let t = Ring . ticketer () . unwrap () ; let mut cipher = t . encrypt (b"hello world") . unwrap () ; assert_eq ! (t . decrypt (& cipher) , Some (b"hello world" . to_vec ())) ; cipher . push (0) ; assert_eq ! (t . decrypt (& cipher) , None) ; } # [test] fn aeadticketer_is_debug_and_producestickets () { use alloc :: format ; use super :: * ; let t = AeadTicketer :: new () . unwrap () ; let expect = format ! ("AeadTicketer {{ alg: {TICKETER_AEAD:?} }}") ; assert_eq ! (format ! ("{t:?}") , expect) ; assert_eq ! (t . lifetime () , Duration :: ZERO) ; } }
};
}
