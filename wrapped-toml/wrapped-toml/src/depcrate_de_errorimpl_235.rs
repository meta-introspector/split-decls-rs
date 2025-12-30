// Generated macro for impl_235 (impl)
macro_rules! Depcrate_de_errorimpl_235 {
() => {
// Module: crate::de::error
// Provides: {"impl_235"}
// Dependencies: {}
# [cfg (feature = "parse")] impl < 'i > toml_parser :: ErrorSink for TomlSink < 'i , Option < Error > > { fn report_error (& mut self , error : toml_parser :: ParseError) { if self . sink . is_none () { let input = self . input . get_or_insert_with (| | alloc :: sync :: Arc :: from (self . source . input ())) ; let error = Error :: new (input . clone () , error) ; self . sink = Some (error) ; } } }
};
}
