// Generated macro for OptionVarULE (struct)
macro_rules! Depcrate_ule_optionOptionVarULE {
() => {
// Module: crate::ule::option
// Provides: {"OptionVarULE"}
// Dependencies: {}
# [doc = " A type allowing one to represent `Option<U>` for [`VarULE`] `U` types."] # [doc = ""] # [doc = " ```rust"] # [doc = " use zerovec::ule::OptionVarULE;"] # [doc = " use zerovec::VarZeroVec;"] # [doc = ""] # [doc = " let mut zv: VarZeroVec<OptionVarULE<str>> = VarZeroVec::new();"] # [doc = ""] # [doc = " zv.make_mut().push(&None::<&str>);"] # [doc = " zv.make_mut().push(&Some(\"hello\"));"] # [doc = " zv.make_mut().push(&Some(\"world\"));"] # [doc = " zv.make_mut().push(&None::<&str>);"] # [doc = ""] # [doc = " assert_eq!(zv.get(0).unwrap().as_ref(), None);"] # [doc = " assert_eq!(zv.get(1).unwrap().as_ref(), Some(\"hello\"));"] # [doc = " ```"] # [repr (C , packed)] pub struct OptionVarULE < U : VarULE + ? Sized > (PhantomData < U > , bool , [u8]) ;
};
}
