macro_rules! deps {
    () => {
        StructTrait!();
        TagType!();
        Container!();
    };
}

macro_rules! serialize_struct_tag_field {
    () => {
        deps!();
        fn serialize_struct_tag_field (cattrs : & attr :: Container , struct_trait : & StructTrait) -> TokenStream { match cattrs . tag () { attr :: TagType :: Internal { tag } => { let type_name = cattrs . name () . serialize_name () ; let func = struct_trait . serialize_field (Span :: call_site ()) ; quote ! { # func (& mut __serde_state , # tag , # type_name) ?; } } _ => quote ! { } , } }
    };
}

serialize_struct_tag_field!()