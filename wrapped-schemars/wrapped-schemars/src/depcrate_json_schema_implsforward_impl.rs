// Generated macro for forward_impl (macro)
macro_rules! Depcrate_json_schema_implsforward_impl {
() => {
// Module: crate::json_schema_impls
// Provides: {"forward_impl"}
// Dependencies: {}
macro_rules ! forward_impl { (($ ($ impl : tt) +) => $ target : ty) => { impl $ ($ impl) + { fn inline_schema () -> bool { <$ target as $ crate :: JsonSchema >:: inline_schema () } fn schema_name () -> alloc :: borrow :: Cow <'static , str > { <$ target as $ crate :: JsonSchema >:: schema_name () } fn schema_id () -> alloc :: borrow :: Cow <'static , str > { <$ target as $ crate :: JsonSchema >:: schema_id () } fn json_schema (generator : & mut $ crate :: SchemaGenerator) -> $ crate :: Schema { <$ target as $ crate :: JsonSchema >:: json_schema (generator) } fn _schemars_private_non_optional_json_schema (generator : & mut $ crate :: SchemaGenerator) -> $ crate :: Schema { # [allow (clippy :: used_underscore_items)] <$ target as $ crate :: JsonSchema >:: _schemars_private_non_optional_json_schema (generator) } fn _schemars_private_is_option () -> bool { # [allow (clippy :: used_underscore_items)] <$ target as $ crate :: JsonSchema >:: _schemars_private_is_option () } } } ; ($ ty : ty => $ target : ty) => { forward_impl ! (($ crate :: JsonSchema for $ ty) => $ target) ; } ; }
};
}
