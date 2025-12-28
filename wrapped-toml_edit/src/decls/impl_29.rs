macro_rules! deps {
    () => {
        TomlError!();
        TomlSink!();
        Document!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl < S : AsRef < str > > Document < S > { # [doc = " Parse a TOML document"] pub fn parse (raw : S) -> Result < Self , crate :: TomlError > { let source = toml_parser :: Source :: new (raw . as_ref ()) ; let mut sink = crate :: error :: TomlSink :: < Option < _ > > :: new (source) ; let doc = crate :: parser :: parse_document (source , & mut sink) ; if let Some (err) = sink . into_inner () { Err (err) } else { Ok (Self { root : doc . root , trailing : doc . trailing , raw , }) } } }
    };
}

impl_29!()