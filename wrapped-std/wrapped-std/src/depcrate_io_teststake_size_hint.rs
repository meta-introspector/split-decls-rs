// Generated macro for take_size_hint (function)
macro_rules! Depcrate_io_teststake_size_hint {
() => {
// Module: crate::io::tests
// Provides: {"take_size_hint"}
// Dependencies: {}
# [test] fn take_size_hint () { let size_hint = (& [1 , 2 , 3]) . take (2) . bytes () . size_hint () ; assert_eq ! (size_hint , (2 , Some (2))) ; let size_hint = (& [1 , 2 , 3]) . take (4) . bytes () . size_hint () ; assert_eq ! (size_hint , (3 , Some (3))) ; let size_hint = io :: repeat (0) . take (3) . bytes () . size_hint () ; assert_eq ! (size_hint , (3 , Some (3))) ; }
};
}
