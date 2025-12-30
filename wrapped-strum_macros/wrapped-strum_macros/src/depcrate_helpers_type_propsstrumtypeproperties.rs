// Generated macro for StrumTypeProperties (struct)
macro_rules! Depcrate_helpers_type_propsStrumTypeProperties {
() => {
// Module: crate::helpers::type_props
// Provides: {"StrumTypeProperties"}
// Dependencies: {}
# [derive (Clone , Default)] pub struct StrumTypeProperties { pub parse_err_ty : Option < Path > , pub parse_err_fn : Option < Path > , pub case_style : Option < CaseStyle > , pub ascii_case_insensitive : bool , pub crate_module_path : Option < Path > , pub discriminant_derives : Vec < Path > , pub discriminant_name : Option < Ident > , pub discriminant_others : Vec < TokenStream > , pub discriminant_vis : Option < Visibility > , pub use_phf : bool , pub prefix : Option < LitStr > , pub suffix : Option < LitStr > , pub enum_repr : Option < TokenStream > , pub const_into_str : bool , pub discriminant_docs : Vec < LitStr > , }
};
}
