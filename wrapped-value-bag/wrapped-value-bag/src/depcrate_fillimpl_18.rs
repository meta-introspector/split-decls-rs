// Generated macro for impl_18 (impl)
macro_rules! Depcrate_fillimpl_18 {
() => {
// Module: crate::fill
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'v > ValueBag < 'v > { # [doc = " Get a value from a fillable slot."] pub const fn from_fill < T > (value : & 'v T) -> Self where T : Fill , { ValueBag { inner : Internal :: Fill (value) , } } }
};
}
