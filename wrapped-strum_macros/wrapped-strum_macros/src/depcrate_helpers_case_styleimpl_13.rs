// Generated macro for impl_13 (impl)
macro_rules! Depcrate_helpers_case_styleimpl_13 {
() => {
// Module: crate::helpers::case_style
// Provides: {"impl_13"}
// Dependencies: {}
impl Parse for CaseStyle { fn parse (input : ParseStream) -> syn :: Result < Self > { let text = input . parse :: < LitStr > () ? ; let val = text . value () ; val . as_str () . parse () . map_err (| _ | { syn :: Error :: new_spanned (& text , format ! ("Unexpected case style for serialize_all: `{}`. Valid values are: `{:?}`" , val , VALID_CASE_STYLES) ,) }) } }
};
}
