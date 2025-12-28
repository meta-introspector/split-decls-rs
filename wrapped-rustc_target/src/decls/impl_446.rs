macro_rules! deps {
    () => {
        ExternAbiWrapper!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl schemars :: JsonSchema for ExternAbiWrapper { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "ExternAbi" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all = rustc_abi :: ExternAbi :: ALL_VARIANTS . iter () . map (| abi | abi . as_str ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all , }) . into () } }
    };
}

impl_446!()