// Generated macro for other_627 (other)
macro_rules! Depcrate_ule_nicheother_627 {
() => {
// Module: crate::ule::niche
// Provides: {"other_627"}
// Dependencies: {}
# [doc = " [`ULE`] type for [`NichedOption<U,N>`] where U implements [`NicheBytes`]."] # [doc = " The invalid bit pattern is used as the niche."] # [doc = ""] # [doc = " This uses 1 byte less than [`crate::ule::OptionULE<U>`] to represent [`NichedOption<U,N>`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use core::num::NonZeroI8;"] # [doc = " use zerovec::ule::NichedOption;"] # [doc = " use zerovec::ZeroVec;"] # [doc = ""] # [doc = " let bytes = &[0x00, 0x01, 0x02, 0x00];"] # [doc = " let zv_no: ZeroVec<NichedOption<NonZeroI8, 1>> ="] # [doc = "     ZeroVec::parse_bytes(bytes).expect(\"Unable to parse as NichedOption.\");"] # [doc = ""] # [doc = " assert_eq!(zv_no.get(0).map(|e| e.0), Some(None));"] # [doc = " assert_eq!(zv_no.get(1).map(|e| e.0), Some(NonZeroI8::new(1)));"] # [doc = " assert_eq!(zv_no.get(2).map(|e| e.0), Some(NonZeroI8::new(2)));"] # [doc = " assert_eq!(zv_no.get(3).map(|e| e.0), Some(None));"] # [doc = " ```"] # [repr (C)] pub union NichedOptionULE < U : NicheBytes < N > + ULE , const N : usize > { # [doc = " Invariant: The value is `niche` only if the bytes equal NICHE_BIT_PATTERN."] niche : [u8 ; N] , # [doc = " Invariant: The value is `valid` if the `niche` field does not match NICHE_BIT_PATTERN."] valid : U , }
};
}
