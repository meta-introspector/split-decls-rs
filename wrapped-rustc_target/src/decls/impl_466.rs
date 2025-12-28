macro_rules! deps {
    () => {
        LinkSelfContainedDefault!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl schemars :: JsonSchema for LinkSelfContainedDefault { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "LinkSelfContainedDefault" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "string" , "enum" : ["false" , "true" , "wasm" , "musl" , "mingw"] }) . into () } }
    };
}

impl_466!()