// Generated macro for impl_108 (impl)
macro_rules! Depcrate_inputimpl_108 {
() => {
// Module: crate::input
// Provides: {"impl_108"}
// Dependencies: {}
impl IntrinsicInput { # [doc = " Extracts all the possible variants as an iterator."] pub fn variants (& self , intrinsic : & Intrinsic ,) -> context :: Result < impl Iterator < Item = InputSet > + '_ > { let mut top_product = vec ! [] ; if ! self . types . is_empty () { top_product . push (self . types . iter () . flat_map (| ty_in | { ty_in . 0 . iter () . map (| v | v . clone () . into_iter ()) . multi_cartesian_product () }) . collect_vec () ,) } if let Ok (mask) = PredicationMask :: try_from (& intrinsic . signature . name) { top_product . push (PredicateForm :: compile_list (& mask , & self . predication_methods) ? . into_iter () . map (| pf | vec ! [InputType :: PredicateForm (pf)]) . collect_vec () ,) } if ! self . n_variant_op . is_empty () { top_product . push (vec ! [vec ! [InputType :: NVariantOp (None)] , vec ! [InputType :: NVariantOp (Some (self . n_variant_op . to_owned ()))] ,]) } let it = top_product . into_iter () . map (| v | v . into_iter ()) . multi_cartesian_product () . filter (| set | ! set . is_empty ()) . map (| set | InputSet (set . into_iter () . flatten () . collect_vec ())) ; Ok (it) } }
};
}
