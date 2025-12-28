macro_rules! deps {
    () => {
        SmallDataThresholdSupport!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl schemars :: JsonSchema for SmallDataThresholdSupport { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "SmallDataThresholdSupport" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "string" , "pattern" : r#"^none|default-for-arch|llvm-module-flag=.+|llvm-arg=.+$"# , }) . into () } }
    };
}

impl_488!()