// Generated macro for rewrite_fn_ptr (function)
macro_rules! Depcrate_typesrewrite_fn_ptr {
() => {
// Module: crate::types
// Provides: {"rewrite_fn_ptr"}
// Dependencies: {}
fn rewrite_fn_ptr (fn_ptr : & ast :: FnPtrTy , span : Span , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { debug ! ("rewrite_bare_fn {:#?}" , shape) ; let mut result = String :: with_capacity (128) ; if let Some (ref lifetime_str) = rewrite_bound_params (context , shape , & fn_ptr . generic_params) { result . push_str ("for<") ; result . push_str (lifetime_str) ; result . push_str ("> ") ; } result . push_str (crate :: utils :: format_safety (fn_ptr . safety)) ; result . push_str (& format_extern (fn_ptr . ext , context . config . force_explicit_abi () ,)) ; result . push_str ("fn") ; let func_ty_shape = if context . use_block_indent () { shape . offset_left (result . len ()) . max_width_error (shape . width , span) ? } else { shape . visual_indent (result . len ()) . sub_width (result . len ()) . max_width_error (shape . width , span) ? } ; let rewrite = format_function_type (fn_ptr . decl . inputs . iter () , & fn_ptr . decl . output , fn_ptr . decl . c_variadic () , span , context , func_ty_shape ,) ? ; result . push_str (& rewrite) ; Ok (result) }
};
}
