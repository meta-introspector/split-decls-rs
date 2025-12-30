// Generated macro for impl_185 (impl)
macro_rules! Depcrate_thin_arcimpl_185 {
() => {
// Module: crate::thin_arc
// Provides: {"impl_185"}
// Dependencies: {}
impl < H , T > Arc < HeaderSliceWithLengthProtected < H , T > > { # [doc = " Converts an `Arc` into a `ThinArc`. This consumes the `Arc`, so the refcount"] # [doc = " is not modified."] # [inline] pub fn protected_into_thin (a : Self) -> ThinArc < H , T > { debug_assert_eq ! (a . length () , a . slice () . len () , "Length needs to be correct for ThinArc to work") ; let fat_ptr : * mut ArcInner < HeaderSliceWithLengthProtected < H , T > > = Arc :: into_raw_inner (a) ; let thin_ptr : * mut ArcInner < HeaderSlice < HeaderWithLength < H > , [T ; 0] > > = fat_ptr . cast () ; ThinArc { ptr : unsafe { ptr :: NonNull :: new_unchecked (thin_ptr) } , phantom : PhantomData , } } # [doc = " Converts a `ThinArc` into an `Arc`. This consumes the `ThinArc`, so the refcount"] # [doc = " is not modified."] # [inline] pub fn protected_from_thin (a : ThinArc < H , T >) -> Self { let a = ManuallyDrop :: new (a) ; let ptr = thin_to_thick (& a) ; unsafe { Arc :: from_raw_inner (ptr) } } # [doc = " Obtains a HeaderSliceWithLengthProtected from an unchecked HeaderSliceWithLengthUnchecked, wrapped in an Arc"] # [doc = ""] # [doc = " # Safety"] # [doc = " Assumes that the header length matches the slice length."] # [inline] unsafe fn from_unprotected_unchecked (a : Arc < HeaderSliceWithLengthUnchecked < H , T > >) -> Self { unsafe { Arc :: from_raw_inner (Arc :: into_raw_inner (a) as _) } } }
};
}
