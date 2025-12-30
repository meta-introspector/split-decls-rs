// Generated macro for impl_1495 (impl)
macro_rules! Depcrate_utilimpl_1495 {
() => {
// Module: crate::util
// Provides: {"impl_1495"}
// Dependencies: {}
impl From < std :: time :: SystemTime > for Timestamp { fn from (system_time : std :: time :: SystemTime) -> Self { let seconds ; let nanoseconds ; match system_time . duration_since (std :: time :: UNIX_EPOCH) { Ok (duration) => { seconds = duration . as_secs () as i64 ; nanoseconds = duration . subsec_nanos () ; } Err (error) => { let negative = error . duration () ; let negative_secs = negative . as_secs () as i64 ; let negative_nanos = negative . subsec_nanos () ; if negative_nanos == 0 { seconds = - negative_secs ; nanoseconds = 0 ; } else { seconds = - 1 - negative_secs ; nanoseconds = NSEC_PER_SEC - negative_nanos ; } } } Self { seconds , nanoseconds , } } }
};
}
