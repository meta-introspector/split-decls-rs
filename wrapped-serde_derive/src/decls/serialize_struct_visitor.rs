macro_rules! deps {
    () => {
        Parameters!();
        StructTrait!();
        Field!();
    };
}

macro_rules! serialize_struct_visitor {
    () => {
        deps!();
        fn serialize_struct_visitor (fields : & [Field] , params : & Parameters , is_enum : bool , struct_trait : & StructTrait ,) -> Vec < TokenStream > { fields . iter () . filter (| & field | ! field . attrs . skip_serializing ()) . map (| field | { let member = & field . member ; let mut field_expr = if is_enum { quote ! (# member) } else { get_member (params , field , member) } ; let key_expr = field . attrs . name () . serialize_name () ; let skip = field . attrs . skip_serializing_if () . map (| path | quote ! (# path (# field_expr))) ; if let Some (path) = field . attrs . serialize_with () { field_expr = wrap_serialize_field_with (params , field . ty , path , & field_expr) ; } let span = field . original . span () ; let ser = if field . attrs . flatten () { let func = quote_spanned ! (span => _serde :: Serialize :: serialize) ; quote ! { # func (&# field_expr , _serde ::# private :: ser :: FlatMapSerializer (& mut __serde_state)) ?; } } else { let func = struct_trait . serialize_field (span) ; quote ! { # func (& mut __serde_state , # key_expr , # field_expr) ?; } } ; match skip { None => ser , Some (skip) => { if let Some (skip_func) = struct_trait . skip_field (span) { quote ! { if !# skip { # ser } else { # skip_func (& mut __serde_state , # key_expr) ?; } } } else { quote ! { if !# skip { # ser } } } } } }) . collect () }
    };
}

serialize_struct_visitor!();