macro_rules! deps {
    () => {
        Field!();
        Container!();
        Parameters!();
        Fragment!();
    };
}

macro_rules! serialize_newtype_struct {
    () => {
        deps!();
        fn serialize_newtype_struct (params : & Parameters , field : & Field , cattrs : & attr :: Container ,) -> Fragment { let type_name = cattrs . name () . serialize_name () ; let mut field_expr = get_member (params , field , & Member :: Unnamed (Index { index : 0 , span : Span :: call_site () , }) ,) ; if let Some (path) = field . attrs . serialize_with () { field_expr = wrap_serialize_field_with (params , field . ty , path , & field_expr) ; } let span = field . original . span () ; let func = quote_spanned ! (span => _serde :: Serializer :: serialize_newtype_struct) ; quote_expr ! { # func (__serializer , # type_name , # field_expr) } }
    };
}

serialize_newtype_struct!();