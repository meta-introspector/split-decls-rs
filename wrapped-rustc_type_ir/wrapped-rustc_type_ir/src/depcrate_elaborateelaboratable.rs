// Generated macro for Elaboratable (trait)
macro_rules! Depcrate_elaborateElaboratable {
() => {
// Module: crate::elaborate
// Provides: {"Elaboratable"}
// Dependencies: {}
# [doc = " Describes how to elaborate an obligation into a sub-obligation."] pub trait Elaboratable < I : Interner > { fn predicate (& self) -> I :: Predicate ; fn child (& self , clause : I :: Clause) -> Self ; fn child_with_derived_cause (& self , clause : I :: Clause , span : I :: Span , parent_trait_pred : ty :: Binder < I , ty :: TraitPredicate < I > > , index : usize ,) -> Self ; }
};
}
