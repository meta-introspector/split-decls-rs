macro_rules! deps {
    () => {
        TupleTrait!();
        Field!();
        Parameters!();
    };
}

macro_rules! serialize_tuple_struct_visitor {
    () => {
        deps!();
        fn serialize_tuple_struct_visitor (fields : & [Field] , params : & Parameters , is_enum : bool , tuple_trait : & TupleTrait ,) -> Vec < TokenStream > { fields . iter () . enumerate () . filter (| (_ , field) | ! field . attrs . skip_serializing ()) . map (| (i , field) | { let mut field_expr = if is_enum { let id = Ident :: new (& format ! ("__field{}" , i) , Span :: call_site ()) ; quote ! (# id) } else { get_member (params , field , & Member :: Unnamed (Index { index : i as u32 , span : Span :: call_site () , }) ,) } ; let skip = field . attrs . skip_serializing_if () . map (| path | quote ! (# path (# field_expr))) ; if let Some (path) = field . attrs . serialize_with () { field_expr = wrap_serialize_field_with (params , field . ty , path , & field_expr) ; } let span = field . original . span () ; let func = tuple_trait . serialize_element (span) ; let ser = quote ! { # func (& mut __serde_state , # field_expr) ?; } ; match skip { None => ser , Some (skip) => quote ! (if !# skip { # ser }) , } }) . collect () }
    };
}

serialize_tuple_struct_visitor!()