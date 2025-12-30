// Generated macro for VarZeroVecFormat (trait)
macro_rules! Depcrate_varzerovec_componentsVarZeroVecFormat {
() => {
// Module: crate::varzerovec::components
// Provides: {"VarZeroVecFormat"}
// Dependencies: {}
# [doc = " This trait allows switching between different possible internal"] # [doc = " representations of VarZeroVec."] # [doc = ""] # [doc = " Currently this crate supports three formats: [`Index8`], [`Index16`] and [`Index32`],"] # [doc = " with [`Index16`] being the default for all [`VarZeroVec`](super::VarZeroVec)"] # [doc = " types unless explicitly specified otherwise."] # [doc = ""] # [doc = " Do not implement this trait, its internals may be changed in the future,"] # [doc = " and all of its associated items are hidden from the docs."] pub trait VarZeroVecFormat : 'static + Sized { # [doc = " The type to use for the indexing array"] # [doc = ""] # [doc = " Safety: must be a ULE for which all byte sequences are allowed"] # [doc (hidden)] type Index : IntegerULE ; # [doc = " The type to use for the length segment"] # [doc = ""] # [doc = " Safety: must be a ULE for which all byte sequences are allowed"] # [doc (hidden)] type Len : IntegerULE ; }
};
}
