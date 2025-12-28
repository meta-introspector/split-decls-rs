macro_rules! lift {
    () => {
        fn lift (mut ty : syn :: Type) -> syn :: Type { struct ItoJ ; impl VisitMut for ItoJ { fn visit_type_path_mut (& mut self , i : & mut syn :: TypePath) { if i . qself . is_none () { if let Some (first) = i . path . segments . first_mut () { if first . ident == "I" { * first = parse_quote ! { J } ; } } } syn :: visit_mut :: visit_type_path_mut (self , i) ; } } ItoJ . visit_type_mut (& mut ty) ; ty }
    };
}

lift!();