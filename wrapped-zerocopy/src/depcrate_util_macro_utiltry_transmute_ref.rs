// Generated macro for try_transmute_ref (function)
macro_rules! Depcrate_util_macro_utiltry_transmute_ref {
() => {
// Module: crate::util::macro_util
// Provides: {"try_transmute_ref"}
// Dependencies: {}
# [doc = " Attempts to transmute `&Src` into `&Dst`."] # [doc = ""] # [doc = " A helper for `try_transmute_ref!`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " `try_transmute_ref` may either produce a post-monomorphization error or a"] # [doc = " panic if `Dst` is bigger or has a stricter alignment requirement than `Src`."] # [doc = " Otherwise, `try_transmute_ref` panics under the same circumstances as"] # [doc = " [`is_bit_valid`]."] # [doc = ""] # [doc = " [`is_bit_valid`]: TryFromBytes::is_bit_valid"] # [inline (always)] pub fn try_transmute_ref < Src , Dst > (src : & Src) -> Result < & Dst , ValidityError < & Src , Dst > > where Src : IntoBytes + Immutable , Dst : TryFromBytes + Immutable , { let ptr = Ptr :: from_ref (src) ; let ptr = ptr . bikeshed_recall_initialized_immutable () ; match try_cast_or_pme :: < Src , Dst , _ , BecauseImmutable , _ > (ptr) { Ok (ptr) => { static_assert ! (Src , Dst => mem :: align_of ::< Dst > () <= mem :: align_of ::< Src > ()) ; let ptr = unsafe { ptr . assume_alignment :: < invariant :: Aligned > () } ; Ok (ptr . as_ref ()) } Err (err) => Err (err . map_src (| ptr | { let ptr = unsafe { ptr . assume_valid () } ; ptr . as_ref () })) , } }
};
}
