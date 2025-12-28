macro_rules! deps {
    () => {
        EndianWrapper!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl schemars :: JsonSchema for EndianWrapper { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "Endian" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "string" , "enum" : ["big" , "little"] }) . into () } }
    };
}

impl_442!();