// Generated macro for pretend_fields_used (function)
macro_rules! Depcrate_pretendpretend_fields_used {
() => {
// Module: crate::pretend
// Provides: {"pretend_fields_used"}
// Dependencies: {}
fn pretend_fields_used (cont : & Container , is_packed : bool) -> TokenStream { match & cont . data { Data :: Enum (variants) => pretend_fields_used_enum (cont , variants) , Data :: Struct (Style :: Struct | Style :: Tuple | Style :: Newtype , fields) => { if is_packed { pretend_fields_used_struct_packed (cont , fields) } else { pretend_fields_used_struct (cont , fields) } } Data :: Struct (Style :: Unit , _) => quote ! () , } }
};
}
