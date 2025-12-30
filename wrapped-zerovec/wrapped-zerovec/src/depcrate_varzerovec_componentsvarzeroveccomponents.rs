// Generated macro for VarZeroVecComponents (struct)
macro_rules! Depcrate_varzerovec_componentsVarZeroVecComponents {
() => {
// Module: crate::varzerovec::components
// Provides: {"VarZeroVecComponents"}
// Dependencies: {}
# [doc = " A more parsed version of `VarZeroSlice`. This type is where most of the VarZeroVec"] # [doc = " internal representation code lies."] # [doc = ""] # [doc = " This is *basically* an `&'a [u8]` to a zero copy buffer, but split out into"] # [doc = " the buffer components. Logically this is capable of behaving as"] # [doc = " a `&'a [T::VarULE]`, but since `T::VarULE` is unsized that type does not actually"] # [doc = " exist."] # [doc = ""] # [doc = " See [`VarZeroVecComponents::parse_bytes()`] for information on the internal invariants involved"] # [derive (Debug)] pub struct VarZeroVecComponents < 'a , T : ? Sized , F > { # [doc = " The number of elements"] len : u32 , # [doc = " The list of indices into the `things` slice"] # [doc = " Since the first element is always at things[0], the first element of the indices array is for the *second* element"] indices : & 'a [u8] , # [doc = " The contiguous list of `T::VarULE`s"] things : & 'a [u8] , marker : PhantomData < (& 'a T , F) > , }
};
}
