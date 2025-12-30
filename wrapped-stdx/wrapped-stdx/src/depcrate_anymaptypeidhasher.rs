// Generated macro for TypeIdHasher (struct)
macro_rules! Depcrate_anymapTypeIdHasher {
() => {
// Module: crate::anymap
// Provides: {"TypeIdHasher"}
// Dependencies: {}
# [doc = " A hasher designed to eke a little more speed out, given `TypeId`'s known characteristics."] # [doc = ""] # [doc = " Specifically, this is a no-op hasher that expects to be fed a u64's worth of"] # [doc = " randomly-distributed bits. It works well for `TypeId` (eliminating start-up time, so that my"] # [doc = " `get_missing` benchmark is ~30ns rather than ~900ns, and being a good deal faster after that, so"] # [doc = " that my `insert_and_get_on_260_types` benchmark is ~12μs instead of ~21.5μs), but will"] # [doc = " panic in debug mode and always emit zeros in release mode for any other sorts of inputs, so"] # [doc = " yeah, don't use it! 😀"] # [derive (Default)] pub struct TypeIdHasher { value : u64 , }
};
}
