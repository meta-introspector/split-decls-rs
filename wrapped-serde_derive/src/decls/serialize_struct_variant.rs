macro_rules! deps {
    () => {
        StructVariant!();
        Parameters!();
        Field!();
        Name!();
        Fragment!();
        StructTrait!();
    };
}

macro_rules! serialize_struct_variant {
    () => {
        deps!();
        fn serialize_struct_variant (context : StructVariant , params : & Parameters , fields : & [Field] , name : & Name ,) -> Fragment { if fields . iter () . any (| field | field . attrs . flatten ()) { return serialize_struct_variant_with_flatten (context , params , fields , name) ; } let struct_trait = match context { StructVariant :: ExternallyTagged { .. } => StructTrait :: SerializeStructVariant , StructVariant :: InternallyTagged { .. } | StructVariant :: Untagged => { StructTrait :: SerializeStruct } } ; let serialize_fields = serialize_struct_visitor (fields , params , true , & struct_trait) ; let mut serialized_fields = fields . iter () . filter (| & field | ! field . attrs . skip_serializing ()) . peekable () ; let let_mut = mut_if (serialized_fields . peek () . is_some ()) ; let len = serialized_fields . map (| field | { let member = & field . member ; match field . attrs . skip_serializing_if () { Some (path) => quote ! (if # path (# member) { 0 } else { 1 }) , None => quote ! (1) , } }) . fold (quote ! (0) , | sum , expr | quote ! (# sum + # expr)) ; match context { StructVariant :: ExternallyTagged { variant_index , variant_name , } => { quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_struct_variant (__serializer , # name , # variant_index , # variant_name , # len ,) ?; # (# serialize_fields) * _serde :: ser :: SerializeStructVariant :: end (__serde_state) } } StructVariant :: InternallyTagged { tag , variant_name } => { quote_block ! { let mut __serde_state = _serde :: Serializer :: serialize_struct (__serializer , # name , # len + 1 ,) ?; _serde :: ser :: SerializeStruct :: serialize_field (& mut __serde_state , # tag , # variant_name ,) ?; # (# serialize_fields) * _serde :: ser :: SerializeStruct :: end (__serde_state) } } StructVariant :: Untagged => { quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_struct (__serializer , # name , # len ,) ?; # (# serialize_fields) * _serde :: ser :: SerializeStruct :: end (__serde_state) } } } }
    };
}

serialize_struct_variant!()