// Generated macro for tests (module)
macro_rules! Depcrate_crypto_aws_lc_rs_ticketertests {
() => {
// Module: crate::crypto::aws_lc_rs::ticketer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: crypto :: TicketerFactory ; use crate :: crypto :: aws_lc_rs :: AwsLcRs ; # [test] fn basic_pairwise_test () { let t = AwsLcRs . ticketer () . unwrap () ; let cipher = t . encrypt (b"hello world") . unwrap () ; let plain = t . decrypt (& cipher) . unwrap () ; assert_eq ! (plain , b"hello world") ; } # [test] fn refuses_decrypt_before_encrypt () { let t = AwsLcRs . ticketer () . unwrap () ; assert_eq ! (t . decrypt (b"hello") , None) ; } # [test] fn refuses_decrypt_larger_than_largest_encryption () { let t = AwsLcRs . ticketer () . unwrap () ; let mut cipher = t . encrypt (b"hello world") . unwrap () ; assert_eq ! (t . decrypt (& cipher) , Some (b"hello world" . to_vec ())) ; cipher . push (0) ; assert_eq ! (t . decrypt (& cipher) , None) ; } # [test] fn rfc5077ticketer_is_debug_and_producestickets () { use alloc :: format ; use super :: * ; let t = Rfc5077Ticketer :: new () . unwrap () ; assert_eq ! (format ! ("{t:?}") , "Rfc5077Ticketer { .. }") ; assert_eq ! (t . lifetime () , Duration :: ZERO) ; } }
};
}
