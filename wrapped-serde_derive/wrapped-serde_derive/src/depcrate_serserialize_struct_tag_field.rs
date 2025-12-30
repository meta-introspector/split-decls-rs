// Generated macro for serialize_struct_tag_field (function)
macro_rules! Depcrate_serserialize_struct_tag_field {
() => {
// Module: crate::ser
// Provides: {"serialize_struct_tag_field"}
// Dependencies: {}
fn serialize_struct_tag_field (cattrs : & attr :: Container , struct_trait : & StructTrait) -> TokenStream { match cattrs . tag () { attr :: TagType :: Internal { tag } => { let type_name = cattrs . name () . serialize_name () ; let func = struct_trait . serialize_field (Span :: call_site ()) ; quote ! { # func (& mut __serde_state , # tag , # type_name) ?; } } _ => quote ! { } , } }
};
}
