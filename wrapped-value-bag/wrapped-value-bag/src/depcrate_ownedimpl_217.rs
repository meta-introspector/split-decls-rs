// Generated macro for impl_217 (impl)
macro_rules! Depcrate_ownedimpl_217 {
() => {
// Module: crate::owned
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'v > ValueBag < 'v > { # [doc = " Buffer this value into an [`OwnedValueBag`]."] pub fn to_owned (& self) -> OwnedValueBag { OwnedValueBag { inner : self . inner . to_owned () , } } # [doc = " Buffer this value into an [`OwnedValueBag`], internally storing it"] # [doc = " in an `Arc` for cheap cloning."] pub fn to_shared (& self) -> OwnedValueBag { self . to_owned () . into_shared () } }
};
}
