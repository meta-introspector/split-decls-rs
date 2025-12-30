// Generated macro for expr_for_adjacent_tagged_enum (function)
macro_rules! Depcrate_schema_exprsexpr_for_adjacent_tagged_enum {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_adjacent_tagged_enum"}
// Dependencies: {}
fn expr_for_adjacent_tagged_enum < 'a > (cont : & Container , variants : impl Iterator < Item = & 'a Variant < 'a > > , tag_name : & str , content_name : & str , deny_unknown_fields : bool ,) -> SchemaExpr { let schemas = variants . map (| variant | { if variant . serde_attrs . untagged () { return (Some (variant) , expr_for_untagged_enum_variant (cont , variant , deny_unknown_fields , true) ,) ; } let content_schema = if variant . is_unit () && variant . attrs . with . is_none () { None } else { Some (expr_for_untagged_enum_variant (cont , variant , deny_unknown_fields , false ,)) } ; let (add_content_to_props , add_content_to_required) = content_schema . map (| content_schema | { (quote ! (# content_name : (# content_schema) ,) , quote ! (# content_name ,) ,) }) . unwrap_or_default () ; let name = variant . name () ; let tag_schema = quote ! { schemars :: json_schema ! ({ "type" : "string" , "const" : # name , }) } ; let set_additional_properties = if deny_unknown_fields { quote ! { "additionalProperties" : false , } } else { TokenStream :: new () } ; let mut outer_schema = SchemaExpr :: from (quote ! (schemars :: json_schema ! ({ "type" : "object" , "properties" : { # tag_name : (# tag_schema) , # add_content_to_props } , "required" : [# tag_name , # add_content_to_required] , # set_additional_properties }))) ; variant . add_mutators (& mut outer_schema . mutators) ; (Some (variant) , outer_schema) }) . collect () ; variant_subschemas (cont , true , schemas) }
};
}
