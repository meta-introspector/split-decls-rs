// Generated macro for tests (module)
macro_rules! Depcrate_ugidtests {
() => {
// Module: crate::ugid
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_sizes () { assert_eq_size ! (RawUid , u32) ; assert_eq_size ! (RawGid , u32) ; assert_eq_size ! (RawUid , libc :: uid_t) ; assert_eq_size ! (RawGid , libc :: gid_t) ; } }
};
}
