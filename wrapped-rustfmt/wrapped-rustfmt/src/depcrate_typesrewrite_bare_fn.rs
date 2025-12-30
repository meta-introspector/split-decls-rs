// Generated macro for rewrite_bare_fn (function)
macro_rules! Depcrate_typesrewrite_bare_fn {
() => {
// Module: crate::types
// Provides: {"rewrite_bare_fn"}
// Dependencies: {}
fn rewrite_bare_fn (bare_fn : & ast :: BareFnTy , span : Span , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { debug ! ("rewrite_bare_fn {:#?}" , shape) ; let mut result = String :: with_capacity (128) ; if let Some (ref lifetime_str) = rewrite_bound_params (context , shape , & bare_fn . generic_params) { result . push_str ("for<") ; result . push_str (lifetime_str) ; result . push_str ("> ") ; } result . push_str (crate :: utils :: format_safety (bare_fn . safety)) ; result . push_str (& format_extern (bare_fn . ext , context . config . force_explicit_abi () ,)) ; result . push_str ("fn") ; let func_ty_shape = if context . use_block_indent () { shape . offset_left (result . len () , span) ? } else { shape . visual_indent (result . len ()) . sub_width (result . len () , span) ? } ; let rewrite = format_function_type (bare_fn . decl . inputs . iter () , & bare_fn . decl . output , bare_fn . decl . c_variadic () , span , context , func_ty_shape ,) ? ; result . push_str (& rewrite) ; Ok (result) }
};
}
