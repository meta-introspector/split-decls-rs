// Generated macro for flags (function)
macro_rules! Depcrateflags {
() => {
// Module: crate
// Provides: {"flags"}
// Dependencies: {}
# [doc = " a bit of a sanity check that nothing weird happened with symbol resolution"] # [cfg (not (miri))] # [cfg (test)] # [test] fn flags () { unsafe { let rs = libz_rs_sys :: zlibCompileFlags () ; let ng = libz_sys :: zlibCompileFlags () ; assert_eq ! (rs & 0b11 , ng & 0b11) ; assert_eq ! ((rs >> 2) & 0b11 , (ng >> 2) & 0b11) ; assert_eq ! ((rs >> 4) & 0b11 , (ng >> 4) & 0b11) ; assert_eq ! ((rs >> 6) & 0b11 , (ng >> 6) & 0b11) ; assert_eq ! (rs & (1 << 8) , ng & (1 << 8)) ; assert_eq ! (rs & (1 << 20) , ng & (1 << 20)) ; assert_eq ! (rs & (0b111 << 24) , ng & (0b111 << 24)) ; assert_eq ! (rs , ng) ; } }
};
}
