// Generated macro for try_transmute_mut (function)
macro_rules! Depcrate_util_macro_utiltry_transmute_mut {
() => {
// Module: crate::util::macro_util
// Provides: {"try_transmute_mut"}
// Dependencies: {}
# [doc = " Attempts to transmute `&mut Src` into `&mut Dst`."] # [doc = ""] # [doc = " A helper for `try_transmute_mut!`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " `try_transmute_mut` may either produce a post-monomorphization error or a"] # [doc = " panic if `Dst` is bigger or has a stricter alignment requirement than `Src`."] # [doc = " Otherwise, `try_transmute_mut` panics under the same circumstances as"] # [doc = " [`is_bit_valid`]."] # [doc = ""] # [doc = " [`is_bit_valid`]: TryFromBytes::is_bit_valid"] # [inline (always)] pub fn try_transmute_mut < Src , Dst > (src : & mut Src) -> Result < & mut Dst , ValidityError < & mut Src , Dst > > where Src : FromBytes + IntoBytes , Dst : TryFromBytes + IntoBytes , { let ptr = Ptr :: from_mut (src) ; let ptr = ptr . bikeshed_recall_initialized_from_bytes () ; match try_cast_or_pme :: < Src , Dst , _ , BecauseExclusive , _ > (ptr) { Ok (ptr) => { static_assert ! (Src , Dst => mem :: align_of ::< Dst > () <= mem :: align_of ::< Src > ()) ; let ptr = unsafe { ptr . assume_alignment :: < invariant :: Aligned > () } ; Ok (ptr . as_mut ()) } Err (err) => { Err (err . map_src (| ptr | ptr . recall_validity :: < _ , (_ , BecauseInvariantsEq) > () . as_mut ())) } } }
};
}
