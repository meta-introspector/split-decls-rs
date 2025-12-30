// Generated macro for OptionULE (struct)
macro_rules! Depcrate_ule_optionOptionULE {
() => {
// Module: crate::ule::option
// Provides: {"OptionULE"}
// Dependencies: {}
# [doc = " This type is the [`ULE`] type for `Option<U>` where `U` is a [`ULE`] type"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use zerovec::ZeroVec;"] # [doc = ""] # [doc = " let z = ZeroVec::alloc_from_slice(&["] # [doc = "     Some('a'),"] # [doc = "     Some('á'),"] # [doc = "     Some('ø'),"] # [doc = "     None,"] # [doc = "     Some('ł'),"] # [doc = " ]);"] # [doc = ""] # [doc = " assert_eq!(z.get(2), Some(Some('ø')));"] # [doc = " assert_eq!(z.get(3), Some(None));"] # [doc = " ```"] # [repr (C , packed)] pub struct OptionULE < U > (bool , MaybeUninit < U >) ;
};
}
