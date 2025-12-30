// Generated macro for EqULE (trait)
macro_rules! Depcrate_uleEqULE {
() => {
// Module: crate::ule
// Provides: {"EqULE"}
// Dependencies: {}
# [doc = " A type whose byte sequence equals the byte sequence of its ULE type on"] # [doc = " little-endian platforms."] # [doc = ""] # [doc = " This enables certain performance optimizations, such as"] # [doc = " [`ZeroVec::try_from_slice`](crate::ZeroVec::try_from_slice)."] # [doc = ""] # [doc = " # Implementation safety"] # [doc = ""] # [doc = " This trait is safe to implement if the type's ULE (as defined by `impl `[`AsULE`]` for T`)"] # [doc = " has an equal byte sequence as the type itself on little-endian platforms; i.e., one where"] # [doc = " `*const T` can be cast to a valid `*const T::ULE`."] pub unsafe trait EqULE : AsULE { }
};
}
