macro_rules! deps {
    () => {
        SanitizerSet!();
    };
}

macro_rules! impl_513 {
    () => {
        deps!();
        impl schemars :: JsonSchema for SanitizerSet { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "SanitizerSet" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all = Self :: all () . iter () . map (| sanitizer | sanitizer . as_str ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all , }) . into () } }
    };
}

impl_513!()