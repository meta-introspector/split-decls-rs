// Generated macro for now (function)
macro_rules! Depcrate_timestampnow {
() => {
// Module: crate::timestamp
// Provides: {"now"}
// Dependencies: {}
# [cfg (all (feature = "std" , miri))] fn now () -> (u64 , u32) { use std :: { sync :: Mutex , time :: Duration } ; static TS : Mutex < u64 > = Mutex :: new (0) ; let ts = Duration :: from_nanos ({ let mut ts = TS . lock () . unwrap () ; * ts += 1 ; * ts }) ; (ts . as_secs () , ts . subsec_nanos ()) }
};
}
