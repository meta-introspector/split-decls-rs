macro_rules! deps {
    () => {
        Parameters!();
        TupleTrait!();
        Fragment!();
        TupleVariant!();
        Field!();
    };
}

macro_rules! serialize_tuple_variant {
    () => {
        deps!();
        fn serialize_tuple_variant (context : TupleVariant , params : & Parameters , fields : & [Field] ,) -> Fragment { let tuple_trait = match context { TupleVariant :: ExternallyTagged { .. } => TupleTrait :: SerializeTupleVariant , TupleVariant :: Untagged => TupleTrait :: SerializeTuple , } ; let serialize_stmts = serialize_tuple_struct_visitor (fields , params , true , & tuple_trait) ; let mut serialized_fields = fields . iter () . enumerate () . filter (| (_ , field) | ! field . attrs . skip_serializing ()) . peekable () ; let let_mut = mut_if (serialized_fields . peek () . is_some ()) ; let len = serialized_fields . map (| (i , field) | match field . attrs . skip_serializing_if () { None => quote ! (1) , Some (path) => { let field_expr = Ident :: new (& format ! ("__field{}" , i) , Span :: call_site ()) ; quote ! (if # path (# field_expr) { 0 } else { 1 }) } }) . fold (quote ! (0) , | sum , expr | quote ! (# sum + # expr)) ; match context { TupleVariant :: ExternallyTagged { type_name , variant_index , variant_name , } => { quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_tuple_variant (__serializer , # type_name , # variant_index , # variant_name , # len) ?; # (# serialize_stmts) * _serde :: ser :: SerializeTupleVariant :: end (__serde_state) } } TupleVariant :: Untagged => { quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_tuple (__serializer , # len) ?; # (# serialize_stmts) * _serde :: ser :: SerializeTuple :: end (__serde_state) } } } }
    };
}

serialize_tuple_variant!()