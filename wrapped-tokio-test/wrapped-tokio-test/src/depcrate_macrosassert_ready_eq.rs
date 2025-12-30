// Generated macro for assert_ready_eq (macro)
macro_rules! Depcrate_macrosassert_ready_eq {
() => {
// Module: crate::macros
// Provides: {"assert_ready_eq"}
// Dependencies: {}
# [doc = " Asserts if a poll is ready and check for equality on the value"] # [doc = ""] # [doc = " This will invoke `panic!` if the provided `Poll` does not evaluate to `Poll::Ready` at"] # [doc = " runtime and the value produced does not partially equal the expected value."] # [doc = ""] # [doc = " # Custom Messages"] # [doc = ""] # [doc = " This macro has a second form, where a custom panic message can be provided with or without"] # [doc = " arguments for formatting."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_util::future;"] # [doc = " use tokio_test::{assert_ready_eq, task};"] # [doc = ""] # [doc = " let mut fut = task::spawn(future::ready(42));"] # [doc = " assert_ready_eq!(fut.poll(), 42);"] # [doc = " ```"] # [macro_export] macro_rules ! assert_ready_eq { ($ e : expr , $ expect : expr) => { let val = $ crate :: assert_ready ! ($ e) ; assert_eq ! (val , $ expect) } ; ($ e : expr , $ expect : expr , $ ($ msg : tt) +) => { let val = $ crate :: assert_ready ! ($ e , $ ($ msg) *) ; assert_eq ! (val , $ expect , $ ($ msg) *) } ; }
};
}
