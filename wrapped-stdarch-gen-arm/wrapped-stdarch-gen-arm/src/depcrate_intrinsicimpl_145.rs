// Generated macro for impl_145 (impl)
macro_rules! Depcrate_intrinsicimpl_145 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_145"}
// Dependencies: {}
impl ToTokens for Signature { fn to_tokens (& self , tokens : & mut TokenStream) { let name_ident = format_ident ! ("{}" , self . fn_name () . to_string ()) ; let arguments = self . arguments . clone () . into_iter () . map (| mut arg | { if arg . kind . vector () . is_some_and (| ty | ty . base_type () . is_bool ()) && self . predicate_needs_conversion { arg . kind = TypeKind :: Vector (VectorType :: make_predicate_from_bitsize (8)) } arg }) . collect_vec () ; let static_defs = & self . static_defs ; tokens . append_all (quote ! { fn # name_ident <# (# static_defs) ,*> (# (# arguments) ,*) }) ; if let Some (ref return_type) = self . return_type { if return_type . vector () . is_some_and (| ty | ty . base_type () . is_bool ()) && self . predicate_needs_conversion { tokens . append_all (quote ! { -> svbool_t }) } else { tokens . append_all (quote ! { -> # return_type }) } } } }
};
}
