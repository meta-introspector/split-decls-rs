// Generated macro for test (module)
macro_rules! Depcrate_randomtest {
() => {
// Module: crate::random
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn basic () { let mut buf = [0 ; 10] ; SecRandom :: default () . copy_bytes (& mut buf) . unwrap () ; } }
};
}
