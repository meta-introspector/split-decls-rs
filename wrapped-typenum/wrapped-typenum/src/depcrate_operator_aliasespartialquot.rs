// Generated macro for PartialQuot (type)
macro_rules! Depcrate_operator_aliasesPartialQuot {
() => {
// Module: crate::operator_aliases
// Provides: {"PartialQuot"}
// Dependencies: {}
# [doc = " Alias for the associated type of"] # [doc = " `PartialDiv`: `PartialQuot<A, B> = <A as PartialDiv<B>>::Output`"] pub type PartialQuot < A , B > = < A as PartialDiv < B > > :: Output ;
};
}
