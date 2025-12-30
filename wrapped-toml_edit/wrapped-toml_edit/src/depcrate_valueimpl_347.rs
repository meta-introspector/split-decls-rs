// Generated macro for impl_347 (impl)
macro_rules! Depcrate_valueimpl_347 {
() => {
// Module: crate::value
// Provides: {"impl_347"}
// Dependencies: {}
# [cfg (feature = "parse")] impl FromStr for Value { type Err = crate :: TomlError ; # [doc = " Parses a value from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let source = toml_parser :: Source :: new (s) ; let mut sink = crate :: error :: TomlSink :: < Option < _ > > :: new (source) ; let mut value = crate :: parser :: parse_value (source , & mut sink) ; if let Some (err) = sink . into_inner () { Err (err) } else { value . decor_mut () . clear () ; value . despan (s) ; Ok (value) } } }
};
}
