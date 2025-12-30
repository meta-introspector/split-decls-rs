// Generated macro for serialize_struct_as_struct (function)
macro_rules! Depcrate_serserialize_struct_as_struct {
() => {
// Module: crate::ser
// Provides: {"serialize_struct_as_struct"}
// Dependencies: {}
fn serialize_struct_as_struct (params : & Parameters , fields : & [Field] , cattrs : & attr :: Container ,) -> Fragment { let serialize_fields = serialize_struct_visitor (fields , params , false , & StructTrait :: SerializeStruct) ; let type_name = cattrs . name () . serialize_name () ; let tag_field = serialize_struct_tag_field (cattrs , & StructTrait :: SerializeStruct) ; let tag_field_exists = ! tag_field . is_empty () ; let mut serialized_fields = fields . iter () . filter (| & field | ! field . attrs . skip_serializing ()) . peekable () ; let let_mut = mut_if (serialized_fields . peek () . is_some () || tag_field_exists) ; let len = serialized_fields . map (| field | match field . attrs . skip_serializing_if () { None => quote ! (1) , Some (path) => { let field_expr = get_member (params , field , & field . member) ; quote ! (if # path (# field_expr) { 0 } else { 1 }) } }) . fold (quote ! (# tag_field_exists as usize) , | sum , expr | quote ! (# sum + # expr) ,) ; quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_struct (__serializer , # type_name , # len) ?; # tag_field # (# serialize_fields) * _serde :: ser :: SerializeStruct :: end (__serde_state) } }
};
}
