// Generated macro for test (module)
macro_rules! Depcrate_page_stacktest {
() => {
// Module: crate::page::stack
// Provides: {"test"}
// Dependencies: {}
# [cfg (all (loom , test))] mod test { use super :: * ; use crate :: { sync :: UnsafeCell , test_util } ; use loom :: thread ; use std :: sync :: Arc ; # [test] fn transfer_stack () { test_util :: run_model ("transfer_stack" , | | { let causalities = [UnsafeCell :: new (999) , UnsafeCell :: new (999)] ; let shared = Arc :: new ((causalities , TransferStack :: < cfg :: DefaultConfig > :: new ())) ; let shared1 = shared . clone () ; let shared2 = shared . clone () ; let t1 = thread :: spawn (move | | { let (causalities , stack) = & * shared1 ; stack . push (0 , | prev | { causalities [0] . with_mut (| c | unsafe { * c = 0 ; }) ; test_println ! ("prev={:#x}" , prev) }) ; }) ; let t2 = thread :: spawn (move | | { let (causalities , stack) = & * shared2 ; stack . push (1 , | prev | { causalities [1] . with_mut (| c | unsafe { * c = 1 ; }) ; test_println ! ("prev={:#x}" , prev) }) ; }) ; let (causalities , stack) = & * shared ; let mut idx = stack . pop_all () ; while idx == None { idx = stack . pop_all () ; thread :: yield_now () ; } let idx = idx . unwrap () ; causalities [idx] . with (| val | unsafe { assert_eq ! (* val , idx , "UnsafeCell write must happen-before index is pushed to the stack!") ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; }) ; } }
};
}
