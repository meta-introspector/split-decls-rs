// Generated macro for test (module)
macro_rules! Depcrate_quic_io_gsotest {
() => {
// Module: crate::quic::io::gso
// Provides: {"test"}
// Dependencies: {}
# [cfg (all (target_os = "linux" , test))] mod test { # [test] # [doc = " If this test begins to fail, it means the implementation of [`Instant`]"] # [doc = " has changed in the std library."] fn instant_zero () { use std :: time :: Instant ; const INSTANT_ZERO : Instant = unsafe { std :: mem :: transmute (0u128) } ; const NANOS_PER_SEC : u128 = 1_000_000_000 ; # [derive (Debug)] struct Timespec { tv_sec : i64 , tv_nsec : u32 , } let now = Instant :: now () ; let now_timespec : Timespec = unsafe { std :: mem :: transmute (now) } ; let ref_elapsed = now . duration_since (INSTANT_ZERO) . as_nanos () ; let raw_elapsed = now_timespec . tv_sec as u128 * NANOS_PER_SEC + now_timespec . tv_nsec as u128 ; assert_eq ! (ref_elapsed , raw_elapsed) ; } }
};
}
