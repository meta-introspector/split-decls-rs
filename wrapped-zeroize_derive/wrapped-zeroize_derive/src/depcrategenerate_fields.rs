// Generated macro for generate_fields (function)
macro_rules! Depcrategenerate_fields {
() => {
// Module: crate
// Provides: {"generate_fields"}
// Dependencies: {}
fn generate_fields (input : & DeriveInput , method : TokenStream) -> TokenStream { let input_id = & input . ident ; let fields : Vec < _ > = match input . data { Data :: Enum (ref enum_) => enum_ . variants . iter () . filter_map (| variant | { if attr_skip (& variant . attrs) { if variant . fields . iter () . any (| field | attr_skip (& field . attrs)) { panic ! ("duplicate #[zeroize] skip flags") } None } else { let variant_id = & variant . ident ; Some ((quote ! { # input_id :: # variant_id } , & variant . fields)) } }) . collect () , Data :: Struct (ref struct_) => vec ! [(quote ! { # input_id } , & struct_ . fields)] , Data :: Union (ref union_) => panic ! ("Cannot generate fields for untagged union {union_:?}") , } ; let arms = fields . into_iter () . map (| (name , fields) | { let method_field = fields . iter () . enumerate () . filter_map (| (n , field) | { if attr_skip (& field . attrs) { None } else { let name = field_ident (n , field) ; Some (quote ! { # name .# method () }) } }) ; let field_bindings = fields . iter () . enumerate () . map (| (n , field) | field_ident (n , field)) ; let binding = match fields { Fields :: Named (_) => quote ! { # name { # (# field_bindings) ,* } } , Fields :: Unnamed (_) => quote ! { # name (# (# field_bindings) ,*) } , Fields :: Unit => quote ! { # name } , } ; quote ! { # [allow (unused_variables)] # binding => { # (# method_field) ;* } } }) ; quote ! { match self { # (# arms) ,* _ => { } } } }
};
}
