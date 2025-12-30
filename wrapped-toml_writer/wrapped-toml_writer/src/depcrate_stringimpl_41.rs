// Generated macro for impl_41 (impl)
macro_rules! Depcrate_stringimpl_41 {
() => {
// Module: crate::string
// Provides: {"impl_41"}
// Dependencies: {}
impl < 's > TomlKeyBuilder < 's > { pub fn new (decoded : & 's str) -> Self { Self { decoded , metrics : KeyMetrics :: calculate (decoded) , } } pub fn as_default (& self) -> TomlKey < 's > { self . as_unquoted () . or_else (| | self . as_basic_pretty ()) . or_else (| | self . as_literal ()) . unwrap_or_else (| | self . as_basic ()) } pub fn as_unquoted (& self) -> Option < TomlKey < 's > > { if self . metrics . unquoted { Some (TomlKey { decoded : self . decoded , encoding : None , }) } else { None } } pub fn as_literal (& self) -> Option < TomlKey < 's > > { if self . metrics . escape_codes || self . metrics . single_quotes { None } else { Some (TomlKey { decoded : self . decoded , encoding : Some (Encoding :: LiteralString) , }) } } pub fn as_basic_pretty (& self) -> Option < TomlKey < 's > > { if self . metrics . escape_codes || self . metrics . escape || self . metrics . double_quotes { None } else { Some (self . as_basic ()) } } pub fn as_basic (& self) -> TomlKey < 's > { TomlKey { decoded : self . decoded , encoding : Some (Encoding :: BasicString) , } } }
};
}
