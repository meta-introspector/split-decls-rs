macro_rules! deps {
    () => {
        LinkerFlavorCli!();
        LinkerFlavor!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl schemars :: JsonSchema for LinkerFlavorCli { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "LinkerFlavor" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all : Vec < & 'static str > = Self :: all () . iter () . map (| flavor | flavor . desc ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all }) . into () } }
    };
}

impl_461!();