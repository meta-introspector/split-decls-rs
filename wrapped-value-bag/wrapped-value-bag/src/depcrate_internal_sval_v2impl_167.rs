// Generated macro for impl_167 (impl)
macro_rules! Depcrate_internal_sval_v2impl_167 {
() => {
// Module: crate::internal::sval::v2
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'v > ValueBag < 'v > { # [doc = " Get a value from a structured type."] # [doc = ""] # [doc = " This method will attempt to capture the given value as a well-known primitive"] # [doc = " before resorting to using its `Value` implementation."] pub fn capture_sval2 < T > (value : & 'v T) -> Self where T : value_bag_sval2 :: lib :: Value + 'static , { Self :: try_capture (value) . unwrap_or (ValueBag { inner : Internal :: Sval2 (value) , }) } # [doc = " Get a value from a structured type without capturing support."] pub const fn from_sval2 < T > (value : & 'v T) -> Self where T : value_bag_sval2 :: lib :: Value , { ValueBag { inner : Internal :: AnonSval2 (value) , } } # [doc = " Get a value from a structured type without capturing support."] # [inline] pub const fn from_dyn_sval2 (value : & 'v dyn Value) -> Self { ValueBag { inner : Internal :: AnonSval2 (value) , } } }
};
}
