// Generated macro for OwnedValueBag (struct)
macro_rules! Depcrate_ownedOwnedValueBag {
() => {
// Module: crate::owned
// Provides: {"OwnedValueBag"}
// Dependencies: {}
# [doc = " A dynamic structured value."] # [doc = ""] # [doc = " This type is an owned variant of [`ValueBag`] that can be"] # [doc = " constructed using its [`to_owned`](struct.ValueBag.html#method.to_owned) method."] # [doc = " `OwnedValueBag`s are suitable for storing and sharing across threads."] # [doc = ""] # [doc = " `OwnedValueBag`s can be inspected by converting back into a regular `ValueBag`"] # [doc = " using the [`by_ref`](#method.by_ref) method."] # [derive (Clone)] pub struct OwnedValueBag { inner : internal :: owned :: OwnedInternal , }
};
}
