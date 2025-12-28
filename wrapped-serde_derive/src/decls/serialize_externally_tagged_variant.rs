macro_rules! deps {
    () => {
        Variant!();
        TupleVariant!();
        Fragment!();
        Parameters!();
        Container!();
        StructVariant!();
        Style!();
    };
}

macro_rules! serialize_externally_tagged_variant {
    () => {
        deps!();
        fn serialize_externally_tagged_variant (params : & Parameters , variant : & Variant , variant_index : u32 , cattrs : & attr :: Container ,) -> Fragment { let type_name = cattrs . name () . serialize_name () ; let variant_name = variant . attrs . name () . serialize_name () ; if let Some (path) = variant . attrs . serialize_with () { let ser = wrap_serialize_variant_with (params , path , variant) ; return quote_expr ! { _serde :: Serializer :: serialize_newtype_variant (__serializer , # type_name , # variant_index , # variant_name , # ser ,) } ; } match effective_style (variant) { Style :: Unit => { quote_expr ! { _serde :: Serializer :: serialize_unit_variant (__serializer , # type_name , # variant_index , # variant_name ,) } } Style :: Newtype => { let field = & variant . fields [0] ; let mut field_expr = quote ! (__field0) ; if let Some (path) = field . attrs . serialize_with () { field_expr = wrap_serialize_field_with (params , field . ty , path , & field_expr) ; } let span = field . original . span () ; let func = quote_spanned ! (span => _serde :: Serializer :: serialize_newtype_variant) ; quote_expr ! { # func (__serializer , # type_name , # variant_index , # variant_name , # field_expr ,) } } Style :: Tuple => serialize_tuple_variant (TupleVariant :: ExternallyTagged { type_name , variant_index , variant_name , } , params , & variant . fields ,) , Style :: Struct => serialize_struct_variant (StructVariant :: ExternallyTagged { variant_index , variant_name , } , params , & variant . fields , type_name ,) , } }
    };
}

serialize_externally_tagged_variant!();