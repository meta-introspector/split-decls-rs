// Generated macro for Shared (struct)
macro_rules! Depcrate_sharedShared {
() => {
// Module: crate::shared
// Provides: {"Shared"}
// Dependencies: {}
# [doc = " [`Shared`] is a reference-counted handle to an instance."] # [doc = ""] # [doc = " The instance is passed to the EBR garbage collector when the last strong reference is dropped."] # [derive (Debug)] pub struct Shared < T > { instance_ptr : NonNull < RefCounted < T > > , }
};
}
