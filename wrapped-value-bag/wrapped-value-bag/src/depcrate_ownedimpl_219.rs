// Generated macro for impl_219 (impl)
macro_rules! Depcrate_ownedimpl_219 {
() => {
// Module: crate::owned
// Provides: {"impl_219"}
// Dependencies: {}
impl OwnedValueBag { # [doc = " Get a regular [`ValueBag`] from this type."] # [doc = ""] # [doc = " Once a `ValueBag` has been buffered, it will behave"] # [doc = " slightly differently when converted back:"] # [doc = ""] # [doc = " - `fmt::Debug` won't use formatting flags."] # [doc = " - `serde::Serialize` will use the text-based representation."] # [doc = " - The original type may change, so downcasting can stop producing results."] pub const fn by_ref (& self) -> ValueBag { ValueBag { inner : self . inner . by_ref () , } } # [doc = " Make this value cheap to clone and share by internally storing it in an `Arc`."] # [doc = ""] # [doc = " If the value is already shared then this method will simply clone it."] pub fn into_shared (self) -> Self { OwnedValueBag { inner : self . inner . into_shared () , } } }
};
}
