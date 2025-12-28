macro_rules! impl_474 {
    () => {
        impl schemars :: JsonSchema for LinkSelfContainedComponents { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "LinkSelfContainedComponents" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all = Self :: all_components () . iter () . map (| component | component . as_str ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all , }) . into () } }
    };
}

impl_474!()