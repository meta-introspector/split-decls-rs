// Generated macro for methods (macro)
macro_rules! Depcrate_test_simd_returnmethods {
() => {
// Module: crate::test_simd_return
// Provides: {"methods"}
// Dependencies: {}
macro_rules ! methods { ($ ($ (# [$ ($ m : tt) *]) * $ name : ident $ (($ padding : expr)) ?: $ ty : ty { $ expr : expr }) *) => { $ (# [test] $ (# [$ ($ m) *]) * fn $ name () { # [allow (non_local_definitions)] impl TestSimdReturn { extern_methods ! (# [unsafe (method ($ name))] fn $ name () -> $ ty ;) ; } let res = TestSimdReturn ::$ name () ; # [allow (unnecessary_transmutes)] let res_bytes = unsafe { core :: mem :: transmute_copy ::<$ ty , [u8 ; { size_of ::<$ ty > () $ (- $ padding) ? }] > (& res) } ; # [allow (unnecessary_transmutes)] let expr_bytes = unsafe { core :: mem :: transmute_copy ::<$ ty , [u8 ; { size_of ::<$ ty > () $ (- $ padding) ? }] > (&$ expr) } ; assert_eq ! (res_bytes , expr_bytes) ; }) * } ; }
};
}
