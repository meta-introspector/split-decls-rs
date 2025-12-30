// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [no_mangle] extern "C" fn main () -> i32 { macro_rules ! test_atomic { ($ ty : ident) => { paste :: paste ! { fn [< test_atomic_ $ ty >] () { __test_atomic ! ($ ty) ; } print ! ("test test_atomic_{} ... " , stringify ! ($ ty)) ; [< test_atomic_ $ ty >] () ; println ! ("ok") ; } } ; } cfg_has_atomic_cas ! { println ! ("target_has_cas: true") ; } cfg_no_atomic_cas ! { println ! ("target_has_cas: false") ; } test_atomic ! (isize) ; test_atomic ! (usize) ; test_atomic ! (i8) ; test_atomic ! (u8) ; test_atomic ! (i16) ; test_atomic ! (u16) ; test_atomic ! (i32) ; test_atomic ! (u32) ; println ! ("Tests finished successfully") ; 0 }
};
}
