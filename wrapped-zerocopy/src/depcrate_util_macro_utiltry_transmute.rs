// Generated macro for try_transmute (function)
macro_rules! Depcrate_util_macro_utiltry_transmute {
() => {
// Module: crate::util::macro_util
// Provides: {"try_transmute"}
// Dependencies: {}
# [doc = " Attempts to transmute `Src` into `Dst`."] # [doc = ""] # [doc = " A helper for `try_transmute!`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " `try_transmute` may either produce a post-monomorphization error or a panic"] # [doc = " if `Dst` is bigger than `Src`. Otherwise, `try_transmute` panics under the"] # [doc = " same circumstances as [`is_bit_valid`]."] # [doc = ""] # [doc = " [`is_bit_valid`]: TryFromBytes::is_bit_valid"] # [inline (always)] pub fn try_transmute < Src , Dst > (src : Src) -> Result < Dst , ValidityError < Src , Dst > > where Src : IntoBytes , Dst : TryFromBytes , { static_assert ! (Src , Dst => mem :: size_of ::< Dst > () == mem :: size_of ::< Src > ()) ; let mu_src = mem :: MaybeUninit :: new (src) ; let mu_src_copy = unsafe { core :: ptr :: read (& mu_src) } ; let mut mu_dst : mem :: MaybeUninit < Dst > = unsafe { crate :: util :: transmute_unchecked (mu_src_copy) } ; let ptr = Ptr :: from_mut (& mut mu_dst) ; let ptr = unsafe { ptr . assume_validity :: < invariant :: Initialized > () } ; let ptr : Ptr < '_ , Dst , _ > = unsafe { ptr . cast_unsized (| ptr : crate :: pointer :: PtrInner < '_ , mem :: MaybeUninit < Dst > > | { ptr . cast_sized () }) } ; if Dst :: is_bit_valid (ptr . forget_aligned ()) { Ok (unsafe { mu_dst . assume_init () }) } else { Err (ValidityError :: new (unsafe { mu_src . assume_init () })) } }
};
}
