// Generated macro for test_order (macro)
macro_rules! Depcrate_scope_teststest_order {
() => {
// Module: crate::scope::tests
// Provides: {"test_order"}
// Dependencies: {}
macro_rules ! test_order { ($ scope : ident => $ spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; pool . install (|| { let vec = Mutex :: new (vec ! []) ; $ scope (| scope | { let vec = & vec ; for i in 0 .. 10 { scope .$ spawn (move | scope | { for j in 0 .. 10 { scope .$ spawn (move | _ | { vec . lock () . unwrap () . push (i * 10 + j) ; }) ; } }) ; } }) ; vec . into_inner () . unwrap () }) } } ; }
};
}
