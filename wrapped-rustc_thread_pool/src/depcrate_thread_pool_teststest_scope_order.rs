// Generated macro for test_scope_order (macro)
macro_rules! Depcrate_thread_pool_teststest_scope_order {
() => {
// Module: crate::thread_pool::tests
// Provides: {"test_scope_order"}
// Dependencies: {}
macro_rules ! test_scope_order { ($ scope : ident => $ spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; pool . install (|| { let vec = Mutex :: new (vec ! []) ; pool .$ scope (| scope | { let vec = & vec ; for i in 0 .. 10 { scope .$ spawn (move | _ | { vec . lock () . unwrap () . push (i) ; }) ; } }) ; vec . into_inner () . unwrap () }) } } ; }
};
}
