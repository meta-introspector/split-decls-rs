macro_rules! deps {
    () => {
        Container!();
        Fragment!();
        Parameters!();
        TupleTrait!();
        Field!();
    };
}

macro_rules! serialize_tuple_struct {
    () => {
        deps!();
        fn serialize_tuple_struct (params : & Parameters , fields : & [Field] , cattrs : & attr :: Container ,) -> Fragment { let serialize_stmts = serialize_tuple_struct_visitor (fields , params , false , & TupleTrait :: SerializeTupleStruct) ; let type_name = cattrs . name () . serialize_name () ; let mut serialized_fields = fields . iter () . enumerate () . filter (| (_ , field) | ! field . attrs . skip_serializing ()) . peekable () ; let let_mut = mut_if (serialized_fields . peek () . is_some ()) ; let len = serialized_fields . map (| (i , field) | match field . attrs . skip_serializing_if () { None => quote ! (1) , Some (path) => { let index = syn :: Index { index : i as u32 , span : Span :: call_site () , } ; let field_expr = get_member (params , field , & Member :: Unnamed (index)) ; quote ! (if # path (# field_expr) { 0 } else { 1 }) } }) . fold (quote ! (0) , | sum , expr | quote ! (# sum + # expr)) ; quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_tuple_struct (__serializer , # type_name , # len) ?; # (# serialize_stmts) * _serde :: ser :: SerializeTupleStruct :: end (__serde_state) } }
    };
}

serialize_tuple_struct!()