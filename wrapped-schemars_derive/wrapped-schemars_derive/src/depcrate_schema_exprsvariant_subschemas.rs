// Generated macro for variant_subschemas (function)
macro_rules! Depcrate_schema_exprsvariant_subschemas {
() => {
// Module: crate::schema_exprs
// Provides: {"variant_subschemas"}
// Dependencies: {}
# [doc = " Callers must determine if all subschemas are mutually exclusive. The current behaviour is to"] # [doc = " assume that variants are mutually exclusive except for untagged enums."] fn variant_subschemas (cont : & Container , mut unique : bool , schemas : Vec < (Option < & Variant > , SchemaExpr) > ,) -> SchemaExpr { if schemas . iter () . any (| (v , _) | v . is_some_and (| v | v . serde_attrs . untagged ())) { unique = false ; } let keyword = if unique { "oneOf" } else { "anyOf" } ; let add_schemas = schemas . into_iter () . map (| (variant , mut schema) | { if cont . attrs . ref_variants { schema = enum_ref_variants (cont , variant , schema) ; } let add = quote ! { enum_values . push (# schema . to_value ()) ; } ; match variant { Some (v) => v . with_contract_check (add) , None => add , } }) ; quote ! ({ let mut map = schemars :: _private :: serde_json :: Map :: new () ; map . insert (# keyword . into () , schemars :: _private :: serde_json :: Value :: Array ({ let mut enum_values = schemars :: _private :: alloc :: vec :: Vec :: new () ; # (# add_schemas) * enum_values }) ,) ; schemars :: Schema :: from (map) }) . into () }
};
}
