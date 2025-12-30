// Generated macro for Owned (struct)
macro_rules! Depcrate_ownedOwned {
() => {
// Module: crate::owned
// Provides: {"Owned"}
// Dependencies: {}
# [doc = " [`Owned`] uniquely owns an instance."] # [doc = ""] # [doc = " The instance is passed to the `EBR` garbage collector when the [`Owned`] is dropped."] # [derive (Debug)] pub struct Owned < T > { instance_ptr : NonNull < RefCounted < T > > , }
};
}
