// Generated macro for CloneableCartablePointerLike (trait)
macro_rules! Depcrate_cartable_ptrCloneableCartablePointerLike {
() => {
// Module: crate::cartable_ptr
// Provides: {"CloneableCartablePointerLike"}
// Dependencies: {}
# [doc = " An object that implements [`CartablePointerLike`] that also"] # [doc = " supports cloning without changing the address of referenced data."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementer safety:"] # [doc = ""] # [doc = " 1. `addref_raw` must create a new owner such that an additional call to"] # [doc = "    `drop_raw` does not create a dangling pointer"] # [doc = " 2. `addref_raw` must not change the address of any referenced data."] pub unsafe trait CloneableCartablePointerLike : CartablePointerLike { # [doc = " Clones this pointer-like."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Caller safety:"] # [doc = ""] # [doc = " 1. The pointer MUST have been returned by this impl's `into_raw`."] # [doc = " 2. The pointer MUST NOT be dangling."] # [doc (hidden)] unsafe fn addref_raw (pointer : NonNull < Self :: Raw >) ; }
};
}
