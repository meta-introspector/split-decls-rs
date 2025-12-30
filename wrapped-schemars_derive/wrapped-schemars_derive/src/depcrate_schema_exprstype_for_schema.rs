// Generated macro for type_for_schema (function)
macro_rules! Depcrate_schema_exprstype_for_schema {
() => {
// Module: crate::schema_exprs
// Provides: {"type_for_schema"}
// Dependencies: {}
fn type_for_schema (cont : & Container , with_attr : & WithAttr) -> (syn :: Type , Option < TokenStream >) { match with_attr { WithAttr :: Type (ty) => (ty . clone () , None) , WithAttr :: Function (fun) => { let cont_name = & cont . ident ; let fn_name = fun . segments . last () . unwrap () . ident . to_string () ; let (impl_generics , ty_generics , where_clause) = cont . generics . split_for_impl () ; let type_def = quote_spanned ! { fun . span () => struct _SchemarsSchemaWithFunction < T : ?:: core :: marker :: Sized > (:: core :: marker :: PhantomData < T >) ; impl # impl_generics schemars :: JsonSchema for _SchemarsSchemaWithFunction <# cont_name # ty_generics > # where_clause { fn inline_schema () -> bool { true } fn schema_name () -> schemars :: _private :: alloc :: borrow :: Cow <'static , str > { schemars :: _private :: alloc :: borrow :: Cow :: Borrowed (# fn_name) } fn schema_id () -> schemars :: _private :: alloc :: borrow :: Cow <'static , str > { schemars :: _private :: alloc :: borrow :: Cow :: Borrowed (:: core :: concat ! ("_SchemarsSchemaWithFunction/" , :: core :: module_path ! () , "/" , :: core :: stringify ! (# fun))) } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { # fun (generator) } } } ; (parse_quote ! (_SchemarsSchemaWithFunction ::<# cont_name # ty_generics >) , Some (type_def) ,) } } }
};
}
