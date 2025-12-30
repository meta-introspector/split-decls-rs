// Generated macro for scope_two (function)
macro_rules! Depcrate_scope_testsscope_two {
() => {
// Module: crate::scope::tests
// Provides: {"scope_two"}
// Dependencies: {}
# [test] fn scope_two () { let counter = & AtomicUsize :: new (0) ; scope (| s | { s . spawn (move | _ | { counter . fetch_add (1 , Ordering :: SeqCst) ; }) ; s . spawn (move | _ | { counter . fetch_add (10 , Ordering :: SeqCst) ; }) ; }) ; let v = counter . load (Ordering :: SeqCst) ; assert_eq ! (v , 11) ; }
};
}
