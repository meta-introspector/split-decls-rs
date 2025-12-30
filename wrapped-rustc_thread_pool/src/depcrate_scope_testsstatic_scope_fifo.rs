// Generated macro for static_scope_fifo (function)
macro_rules! Depcrate_scope_testsstatic_scope_fifo {
() => {
// Module: crate::scope::tests
// Provides: {"static_scope_fifo"}
// Dependencies: {}
# [test] fn static_scope_fifo () { static COUNTER : AtomicUsize = AtomicUsize :: new (0) ; let mut range = 0 .. 100 ; let sum = range . clone () . sum () ; let iter = & mut range ; COUNTER . store (0 , Ordering :: Relaxed) ; scope_fifo (| s : & ScopeFifo < 'static > | { for i in iter { s . spawn_fifo (move | _ | { COUNTER . fetch_add (i , Ordering :: Relaxed) ; }) ; } }) ; assert_eq ! (COUNTER . load (Ordering :: Relaxed) , sum) ; }
};
}
