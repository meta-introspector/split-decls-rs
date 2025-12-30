// Generated macro for millis (function)
macro_rules! Depcrate_hedgemillis {
() => {
// Module: crate::hedge
// Provides: {"millis"}
// Dependencies: {}
fn millis (duration : Duration) -> u64 { let millis = (duration . subsec_nanos () + NANOS_PER_MILLI - 1) / NANOS_PER_MILLI ; duration . as_secs () . saturating_mul (MILLIS_PER_SEC) . saturating_add (u64 :: from (millis)) }
};
}
