// Generated macro for assert_ok (macro)
macro_rules! Depcrate_macrosassert_ok {
() => {
// Module: crate::macros
// Provides: {"assert_ok"}
// Dependencies: {}
# [doc = " Asserts that the expression evaluates to `Ok` and returns the value."] # [doc = ""] # [doc = " This will invoke the `panic!` macro if the provided expression does not evaluate to `Ok` at"] # [doc = " runtime."] # [doc = ""] # [doc = " # Custom Messages"] # [doc = ""] # [doc = " This macro has a second form, where a custom panic message can be provided with or without"] # [doc = " arguments for formatting."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tokio_test::assert_ok;"] # [doc = ""] # [doc = " let n: u32 = assert_ok!(\"123\".parse());"] # [doc = ""] # [doc = " let s = \"123\";"] # [doc = " let n: u32 = assert_ok!(s.parse(), \"testing parsing {:?} as a u32\", s);"] # [doc = " ```"] # [macro_export] macro_rules ! assert_ok { ($ e : expr) => { assert_ok ! ($ e ,) } ; ($ e : expr ,) => { { use std :: result :: Result ::*; match $ e { Ok (v) => v , Err (e) => panic ! ("assertion failed: Err({:?})" , e) , } } } ; ($ e : expr , $ ($ arg : tt) +) => { { use std :: result :: Result ::*; match $ e { Ok (v) => v , Err (e) => panic ! ("assertion failed: Err({:?}): {}" , e , format_args ! ($ ($ arg) +)) , } } } ; }
};
}
