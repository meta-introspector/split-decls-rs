// Generated macro for relate_args_invariantly (function)
macro_rules! Depcrate_relaterelate_args_invariantly {
() => {
// Module: crate::relate
// Provides: {"relate_args_invariantly"}
// Dependencies: {}
# [inline] pub fn relate_args_invariantly < I : Interner , R : TypeRelation < I > > (relation : & mut R , a_arg : I :: GenericArgs , b_arg : I :: GenericArgs ,) -> RelateResult < I , I :: GenericArgs > { relation . cx () . mk_args_from_iter (iter :: zip (a_arg . iter () , b_arg . iter ()) . map (| (a , b) | { relation . relate_with_variance (ty :: Invariant , VarianceDiagInfo :: default () , a , b) })) }
};
}
