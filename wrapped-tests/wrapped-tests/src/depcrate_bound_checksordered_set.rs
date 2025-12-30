// Generated macro for ordered_set (function)
macro_rules! Depcrate_bound_checksordered_set {
() => {
// Module: crate::bound_checks
// Provides: {"ordered_set"}
// Dependencies: {}
# [test] fn ordered_set () { let set = NSOrderedSet :: < NSObject > :: new () ; assert_throws ("index 0 beyond bounds for empty ordered set" , | | { set . objectAtIndex (0) }) ; }
};
}
