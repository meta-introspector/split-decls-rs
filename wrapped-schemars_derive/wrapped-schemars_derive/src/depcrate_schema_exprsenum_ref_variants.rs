// Generated macro for enum_ref_variants (function)
macro_rules! Depcrate_schema_exprsenum_ref_variants {
() => {
// Module: crate::schema_exprs
// Provides: {"enum_ref_variants"}
// Dependencies: {}
fn enum_ref_variants (cont : & Container , variant : Option < & Variant > , expr : SchemaExpr) -> SchemaExpr { let Some (variant) = variant else { return expr ; } ; let cont_name = & cont . ident ; let variant_name = variant . serde_attrs . name () . deserialize_name () ; let (impl_generics , ty_generics , where_clause) = cont . generics . split_for_impl () ; let type_def = quote ! { struct _SchemarsRefVariant < T : ?:: core :: marker :: Sized > (:: core :: marker :: PhantomData < T >) ; impl # impl_generics schemars :: JsonSchema for _SchemarsRefVariant <# cont_name # ty_generics > # where_clause { fn inline_schema () -> bool { false } fn schema_name () -> schemars :: _private :: alloc :: borrow :: Cow <'static , str > { schemars :: _private :: alloc :: borrow :: Cow :: Borrowed (# variant_name) } fn schema_id () -> schemars :: _private :: alloc :: borrow :: Cow <'static , str > { schemars :: _private :: alloc :: borrow :: Cow :: Owned (schemars :: _private :: alloc :: format ! ("_SchemarsRefVariant/{}::{}" , <# cont_name # ty_generics as schemars :: JsonSchema >:: schema_id () , # variant_name ,)) } fn json_schema (# GENERATOR : & mut schemars :: SchemaGenerator) -> schemars :: Schema { # expr } } } ; let mut expr = SchemaExpr :: from (quote ! (# GENERATOR . subschema_for ::< _SchemarsRefVariant ::<# cont_name # ty_generics >> ()) ,) ; expr . definitions . push (type_def) ; expr }
};
}
