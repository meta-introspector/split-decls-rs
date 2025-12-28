macro_rules! deps {
    () => {
        Fragment!();
        Container!();
        Field!();
        StructTrait!();
        Parameters!();
    };
}

macro_rules! serialize_struct_as_map {
    () => {
        deps!();
        fn serialize_struct_as_map (params : & Parameters , fields : & [Field] , cattrs : & attr :: Container ,) -> Fragment { let serialize_fields = serialize_struct_visitor (fields , params , false , & StructTrait :: SerializeMap) ; let tag_field = serialize_struct_tag_field (cattrs , & StructTrait :: SerializeMap) ; let tag_field_exists = ! tag_field . is_empty () ; let mut serialized_fields = fields . iter () . filter (| & field | ! field . attrs . skip_serializing ()) . peekable () ; let let_mut = mut_if (serialized_fields . peek () . is_some () || tag_field_exists) ; quote_block ! { let # let_mut __serde_state = _serde :: Serializer :: serialize_map (__serializer , _serde ::# private :: None) ?; # tag_field # (# serialize_fields) * _serde :: ser :: SerializeMap :: end (__serde_state) } }
    };
}

serialize_struct_as_map!()