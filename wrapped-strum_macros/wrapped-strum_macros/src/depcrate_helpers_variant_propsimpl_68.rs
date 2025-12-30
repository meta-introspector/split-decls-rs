// Generated macro for impl_68 (impl)
macro_rules! Depcrate_helpers_variant_propsimpl_68 {
() => {
// Module: crate::helpers::variant_props
// Provides: {"impl_68"}
// Dependencies: {}
impl StrumVariantProperties { fn ident_as_str (& self , case_style : Option < CaseStyle >) -> LitStr { let ident = self . ident . as_ref () . expect ("identifier") ; LitStr :: new (& ident . convert_case (case_style) , ident . span ()) } pub fn get_preferred_name (& self , case_style : Option < CaseStyle > , prefix : Option < & LitStr > , suffix : Option < & LitStr > ,) -> LitStr { let mut output = self . to_string . as_ref () . cloned () . unwrap_or_else (| | { self . serialize . iter () . max_by_key (| s | s . value () . len ()) . cloned () . unwrap_or_else (| | self . ident_as_str (case_style)) }) ; if let Some (prefix) = prefix { output = LitStr :: new (& (prefix . value () + & output . value ()) , output . span ()) ; } if let Some (suffix) = suffix { output = LitStr :: new (& (output . value () + & suffix . value ()) , output . span ()) ; } output } pub fn get_serializations (& self , case_style : Option < CaseStyle >) -> Vec < LitStr > { let mut attrs = self . serialize . clone () ; if let Some (to_string) = & self . to_string { attrs . push (to_string . clone ()) ; } if attrs . is_empty () { attrs . push (self . ident_as_str (case_style)) ; } attrs } }
};
}
