macro_rules! deps {
    () => {
        Parameters!();
        Container!();
        Style!();
        StructVariant!();
        Variant!();
        Fragment!();
    };
}

macro_rules! serialize_internally_tagged_variant {
    () => {
        deps!();
        fn serialize_internally_tagged_variant (params : & Parameters , variant : & Variant , cattrs : & attr :: Container , tag : & str ,) -> Fragment { let type_name = cattrs . name () . serialize_name () ; let variant_name = variant . attrs . name () . serialize_name () ; let enum_ident_str = params . type_name () ; let variant_ident_str = variant . ident . to_string () ; if let Some (path) = variant . attrs . serialize_with () { let ser = wrap_serialize_variant_with (params , path , variant) ; return quote_expr ! { _serde ::# private :: ser :: serialize_tagged_newtype (__serializer , # enum_ident_str , # variant_ident_str , # tag , # variant_name , # ser ,) } ; } match effective_style (variant) { Style :: Unit => { quote_block ! { let mut __struct = _serde :: Serializer :: serialize_struct (__serializer , # type_name , 1) ?; _serde :: ser :: SerializeStruct :: serialize_field (& mut __struct , # tag , # variant_name) ?; _serde :: ser :: SerializeStruct :: end (__struct) } } Style :: Newtype => { let field = & variant . fields [0] ; let mut field_expr = quote ! (__field0) ; if let Some (path) = field . attrs . serialize_with () { field_expr = wrap_serialize_field_with (params , field . ty , path , & field_expr) ; } let span = field . original . span () ; let func = quote_spanned ! (span => _serde ::# private :: ser :: serialize_tagged_newtype) ; quote_expr ! { # func (__serializer , # enum_ident_str , # variant_ident_str , # tag , # variant_name , # field_expr ,) } } Style :: Struct => serialize_struct_variant (StructVariant :: InternallyTagged { tag , variant_name } , params , & variant . fields , type_name ,) , Style :: Tuple => unreachable ! ("checked in serde_derive_internals") , } }
    };
}

serialize_internally_tagged_variant!();