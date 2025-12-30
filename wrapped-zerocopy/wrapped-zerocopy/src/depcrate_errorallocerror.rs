// Generated macro for AllocError (struct)
macro_rules! Depcrate_errorAllocError {
() => {
// Module: crate::error
// Provides: {"AllocError"}
// Dependencies: {}
# [doc = " The error type of a failed allocation."] # [doc = ""] # [doc = " This type is intended to be deprecated in favor of the standard library's"] # [doc = " [`AllocError`] type once it is stabilized. When that happens, this type will"] # [doc = " be replaced by a type alias to the standard library type. We do not intend"] # [doc = " to treat this as a breaking change; users who wish to avoid breakage should"] # [doc = " avoid writing code which assumes that this is *not* such an alias. For"] # [doc = " example, implementing the same trait for both types will result in an impl"] # [doc = " conflict once this type is an alias."] # [doc = ""] # [doc = " [`AllocError`]: https://doc.rust-lang.org/alloc/alloc/struct.AllocError.html"] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub struct AllocError ;
};
}
