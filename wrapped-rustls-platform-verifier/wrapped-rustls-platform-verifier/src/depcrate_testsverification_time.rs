// Generated macro for verification_time (function)
macro_rules! Depcrate_testsverification_time {
() => {
// Module: crate::tests
// Provides: {"verification_time"}
// Dependencies: {}
# [doc = " Return a fixed [`pki_types::UnixTime`] for certificate validation purposes."] # [doc = ""] # [doc = " We fix the \"now\" value used for certificate validation to a fixed point in time at which"] # [doc = " we know the test certificates are valid. This must be updated if the mock certificates"] # [doc = " are regenerated."] pub (crate) fn verification_time () -> pki_types :: UnixTime { pki_types :: UnixTime :: since_unix_epoch (Duration :: from_secs (1_755_113_506)) }
};
}
