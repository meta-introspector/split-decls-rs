// Generated macro for impl_184 (impl)
macro_rules! Depcrate_thin_arcimpl_184 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_184"}
// Dependencies: {}
impl < H , T > Arc < HeaderSliceWithLengthUnchecked < H , T > > { # [doc = " Converts an `Arc` into a `ThinArc`. This consumes the `Arc`, so the refcount"] # [doc = " is not modified."] # [doc = ""] # [doc = " # Safety"] # [doc = " Assumes that the header length matches the slice length."] # [inline] unsafe fn into_thin_unchecked (a : Self) -> ThinArc < H , T > { let this_protected : Arc < HeaderSliceWithLengthProtected < H , T > > = unsafe { Arc :: from_unprotected_unchecked (a) } ; Arc :: protected_into_thin (this_protected) } # [doc = " Converts an `Arc` into a `ThinArc`. This consumes the `Arc`, so the refcount"] # [doc = " is not modified."] # [inline] pub fn into_thin (a : Self) -> ThinArc < H , T > { assert_eq ! (a . header . length , a . slice . len () , "Length needs to be correct for ThinArc to work") ; unsafe { Self :: into_thin_unchecked (a) } } # [doc = " Converts a `ThinArc` into an `Arc`. This consumes the `ThinArc`, so the refcount"] # [doc = " is not modified."] # [inline] pub fn from_thin (a : ThinArc < H , T >) -> Self { Self :: from_protected (Arc :: < HeaderSliceWithLengthProtected < H , T > > :: protected_from_thin (a)) } # [doc = " Converts an `Arc` into a `ThinArc`. This consumes the `Arc`, so the refcount"] # [doc = " is not modified."] # [inline] fn from_protected (a : Arc < HeaderSliceWithLengthProtected < H , T > >) -> Self { unsafe { Arc :: from_raw_inner (Arc :: into_raw_inner (a) as _) } } }
};
}
