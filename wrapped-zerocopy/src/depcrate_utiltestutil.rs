// Generated macro for testutil (module)
macro_rules! Depcrate_utiltestutil {
() => {
// Module: crate::util
// Provides: {"testutil"}
// Dependencies: {}
# [cfg (test)] pub (crate) mod testutil { use crate :: * ; # [doc = " A `T` which is aligned to at least `align_of::<A>()`."] # [derive (Default)] pub (crate) struct Align < T , A > { pub (crate) t : T , _a : [A ; 0] , } impl < T : Default , A > Align < T , A > { pub (crate) fn set_default (& mut self) { self . t = T :: default () ; } } impl < T , A > Align < T , A > { pub (crate) const fn new (t : T) -> Align < T , A > { Align { t , _a : [] } } } # [doc = " A `T` which is guaranteed not to satisfy `align_of::<A>()`."] # [doc = ""] # [doc = " It must be the case that `align_of::<T>() < align_of::<A>()` in order"] # [doc = " for this type to work properly."] # [repr (C)] pub (crate) struct ForceUnalign < T : Unaligned , A > { _u : u8 , pub (crate) t : T , _a : [A ; 0] , } impl < T : Unaligned , A > ForceUnalign < T , A > { pub (crate) fn new (t : T) -> ForceUnalign < T , A > { ForceUnalign { _u : 0 , t , _a : [] } } } # [derive (KnownLayout , Immutable , FromBytes , IntoBytes , Eq , PartialEq , Ord , PartialOrd , Default , Debug , Copy , Clone ,)] # [repr (C , align (8))] pub (crate) struct AU64 (pub (crate) u64) ; impl AU64 { pub (crate) fn to_bytes (self) -> [u8 ; 8] { crate :: transmute ! (self) } } impl Display for AU64 { # [cfg_attr (all (coverage_nightly , __ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS) , coverage (off))] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (& self . 0 , f) } } }
};
}
