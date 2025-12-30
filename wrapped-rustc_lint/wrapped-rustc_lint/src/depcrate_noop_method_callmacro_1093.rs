// Generated macro for macro_1093 (macro)
macro_rules! Depcrate_noop_method_callmacro_1093 {
() => {
// Module: crate::noop_method_call
// Provides: {"macro_1093"}
// Dependencies: {}
declare_lint ! { # [doc = " The `noop_method_call` lint detects specific calls to noop methods"] # [doc = " such as a calling `<&T as Clone>::clone` where `T: !Clone`."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #![allow(unused)]"] # [doc = " struct Foo;"] # [doc = " let foo = &Foo;"] # [doc = " let clone: &Foo = foo.clone();"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Some method calls are noops meaning that they do nothing. Usually such methods"] # [doc = " are the result of blanket implementations that happen to create some method invocations"] # [doc = " that end up not doing anything. For instance, `Clone` is implemented on all `&T`, but"] # [doc = " calling `clone` on a `&T` where `T` does not implement clone, actually doesn't do anything"] # [doc = " as references are copy. This lint detects these calls and warns the user about them."] pub NOOP_METHOD_CALL , Warn , "detects the use of well-known noop methods" }
};
}
