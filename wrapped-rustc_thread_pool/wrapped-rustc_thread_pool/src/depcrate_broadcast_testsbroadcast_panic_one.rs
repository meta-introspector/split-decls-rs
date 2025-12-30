// Generated macro for broadcast_panic_one (function)
macro_rules! Depcrate_broadcast_testsbroadcast_panic_one {
() => {
// Module: crate::broadcast::tests
// Provides: {"broadcast_panic_one"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn broadcast_panic_one () { let count = AtomicUsize :: new (0) ; let pool = ThreadPoolBuilder :: new () . num_threads (7) . build () . unwrap () ; let result = crate :: unwind :: halt_unwinding (| | { pool . broadcast (| ctx | { count . fetch_add (1 , Ordering :: Relaxed) ; if ctx . index () == 3 { panic ! ("Hello, world!") ; } }) }) ; assert_eq ! (count . into_inner () , 7) ; assert ! (result . is_err () , "broadcast panic should propagate!") ; }
};
}
