// Generated macro for impl_41 (impl)
macro_rules! Depcrate_documentimpl_41 {
() => {
// Module: crate::document
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "parse")] impl < S : AsRef < str > > Document < S > { # [doc = " Parse a TOML document"] pub fn parse (raw : S) -> Result < Self , crate :: TomlError > { let source = toml_parser :: Source :: new (raw . as_ref ()) ; let mut sink = crate :: error :: TomlSink :: < Option < _ > > :: new (source) ; let doc = crate :: parser :: parse_document (source , & mut sink) ; if let Some (err) = sink . into_inner () { Err (err) } else { Ok (Self { root : doc . root , trailing : doc . trailing , raw , }) } } }
};
}
