// Generated macro for impl_100 (impl)
macro_rules! Depcrate_inputimpl_100 {
() => {
// Module: crate::input
// Provides: {"impl_100"}
// Dependencies: {}
impl Ord for InputType { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { use std :: cmp :: Ordering :: * ; match (self , other) { (InputType :: PredicateForm (pf1) , InputType :: PredicateForm (pf2)) => pf1 . cmp (pf2) , (InputType :: Type (ty1) , InputType :: Type (ty2)) => ty1 . cmp (ty2) , (InputType :: NVariantOp (None) , InputType :: NVariantOp (Some (..))) => Less , (InputType :: NVariantOp (Some (..)) , InputType :: NVariantOp (None)) => Greater , (InputType :: NVariantOp (_) , InputType :: NVariantOp (_)) => Equal , (InputType :: Type (..) , InputType :: PredicateForm (..)) => Less , (InputType :: PredicateForm (..) , InputType :: Type (..)) => Greater , (InputType :: Type (..) , InputType :: NVariantOp (..)) => Less , (InputType :: NVariantOp (..) , InputType :: Type (..)) => Greater , (InputType :: PredicateForm (..) , InputType :: NVariantOp (..)) => Less , (InputType :: NVariantOp (..) , InputType :: PredicateForm (..)) => Greater , } } }
};
}
