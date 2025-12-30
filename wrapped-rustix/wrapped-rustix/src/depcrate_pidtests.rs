// Generated macro for tests (module)
macro_rules! Depcrate_pidtests {
() => {
// Module: crate::pid
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_sizes () { use core :: mem :: transmute ; assert_eq_size ! (RawPid , NonZeroI32) ; assert_eq_size ! (RawPid , Pid) ; assert_eq_size ! (RawPid , Option < Pid >) ; const_assert_eq ! (0 as RawPid , unsafe { transmute ::< Option < Pid >, RawPid > (None) }) ; const_assert_eq ! (4567 as RawPid , unsafe { transmute ::< Option < Pid >, RawPid > (Some (Pid :: from_raw_unchecked (4567))) }) ; } # [test] fn test_ctors () { use std :: num :: NonZeroI32 ; assert ! (Pid :: from_raw (0) . is_none ()) ; assert_eq ! (Pid :: from_raw (77) . unwrap () . as_raw_nonzero () , NonZeroI32 :: new (77) . unwrap ()) ; assert_eq ! (Pid :: from_raw (77) . unwrap () . as_raw_pid () , 77) ; assert_eq ! (Pid :: as_raw (Pid :: from_raw (77)) , 77) ; } # [test] fn test_specials () { assert ! (Pid :: from_raw (1) . unwrap () . is_init ()) ; assert_eq ! (Pid :: from_raw (1) . unwrap () , Pid :: INIT) ; } }
};
}
