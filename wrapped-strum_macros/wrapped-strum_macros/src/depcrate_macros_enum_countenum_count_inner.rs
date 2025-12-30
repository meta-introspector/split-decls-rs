// Generated macro for enum_count_inner (function)
macro_rules! Depcrate_macros_enum_countenum_count_inner {
() => {
// Module: crate::macros::enum_count
// Provides: {"enum_count_inner"}
// Dependencies: {}
pub (crate) fn enum_count_inner (ast : & DeriveInput) -> syn :: Result < TokenStream > { let n = match & ast . data { Data :: Enum (v) => v . variants . iter () . try_fold (0usize , | acc , v | { if v . get_variant_properties () ? . disabled . is_none () { Ok :: < usize , syn :: Error > (acc + 1usize) } else { Ok :: < usize , syn :: Error > (acc) } }) ? , _ => return Err (non_enum_error ()) , } ; let type_properties = ast . get_type_properties () ? ; let strum_module_path = type_properties . crate_module_path () ; let name = & ast . ident ; let (impl_generics , ty_generics , where_clause) = ast . generics . split_for_impl () ; Ok (quote ! { # [automatically_derived] impl # impl_generics # strum_module_path :: EnumCount for # name # ty_generics # where_clause { const COUNT : usize = # n ; } }) }
};
}
