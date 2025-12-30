// Generated macro for IntoWasmClosure (trait)
macro_rules! Depcrate_closureIntoWasmClosure {
() => {
// Module: crate::closure
// Provides: {"IntoWasmClosure"}
// Dependencies: {}
# [doc = " An internal trait for the `Closure` type."] # [doc = ""] # [doc = " This trait is not stable and it's not recommended to use this in bounds or"] # [doc = " implement yourself."] # [doc (hidden)] pub trait IntoWasmClosure < T : ? Sized > { fn unsize (self : Box < Self >) -> Box < T > ; }
};
}
