// Generated macro for impl_146 (impl)
macro_rules! Depcrate_internal_serde_v1impl_146 {
() => {
// Module: crate::internal::serde::v1
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'v > ValueBag < 'v > { # [doc = " Get a value from a structured type."] # [doc = ""] # [doc = " This method will attempt to capture the given value as a well-known primitive"] # [doc = " before resorting to using its `Value` implementation."] pub fn capture_serde1 < T > (value : & 'v T) -> Self where T : value_bag_serde1 :: lib :: Serialize + 'static , { Self :: try_capture (value) . unwrap_or (ValueBag { inner : Internal :: Serde1 (value) , }) } # [doc = " Get a value from a structured type without capturing support."] pub const fn from_serde1 < T > (value : & 'v T) -> Self where T : value_bag_serde1 :: lib :: Serialize , { ValueBag { inner : Internal :: AnonSerde1 (value) , } } pub (crate) const fn from_dyn_serde1 (value : & 'v dyn Serialize) -> Self { ValueBag { inner : Internal :: AnonSerde1 (value) , } } }
};
}
