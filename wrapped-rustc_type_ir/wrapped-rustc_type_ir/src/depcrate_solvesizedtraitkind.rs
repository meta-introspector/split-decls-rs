// Generated macro for SizedTraitKind (enum)
macro_rules! Depcrate_solveSizedTraitKind {
() => {
// Module: crate::solve
// Provides: {"SizedTraitKind"}
// Dependencies: {}
# [doc = " Which sizedness trait - `Sized`, `MetaSized`? `PointeeSized` is omitted as it is removed during"] # [doc = " lowering."] # [derive (Copy , Clone , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum SizedTraitKind { # [doc = " `Sized` trait"] Sized , # [doc = " `MetaSized` trait"] MetaSized , }
};
}
