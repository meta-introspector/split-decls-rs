macro_rules! deps {
    () => {
        Fragment!();
        Style!();
        Parameters!();
        Variant!();
        TupleVariant!();
        Container!();
        StructVariant!();
    };
}

macro_rules! serialize_untagged_variant {
    () => {
        deps!();
        fn serialize_untagged_variant (params : & Parameters , variant : & Variant , cattrs : & attr :: Container ,) -> Fragment { if let Some (path) = variant . attrs . serialize_with () { let ser = wrap_serialize_variant_with (params , path , variant) ; return quote_expr ! { _serde :: Serialize :: serialize (# ser , __serializer) } ; } match effective_style (variant) { Style :: Unit => { quote_expr ! { _serde :: Serializer :: serialize_unit (__serializer) } } Style :: Newtype => { let field = & variant . fields [0] ; let mut field_expr = quote ! (__field0) ; if let Some (path) = field . attrs . serialize_with () { field_expr = wrap_serialize_field_with (params , field . ty , path , & field_expr) ; } let span = field . original . span () ; let func = quote_spanned ! (span => _serde :: Serialize :: serialize) ; quote_expr ! { # func (# field_expr , __serializer) } } Style :: Tuple => serialize_tuple_variant (TupleVariant :: Untagged , params , & variant . fields) , Style :: Struct => { let type_name = cattrs . name () . serialize_name () ; serialize_struct_variant (StructVariant :: Untagged , params , & variant . fields , type_name) } } }
    };
}

serialize_untagged_variant!();