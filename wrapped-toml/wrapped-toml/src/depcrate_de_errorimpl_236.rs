// Generated macro for impl_236 (impl)
macro_rules! Depcrate_de_errorimpl_236 {
() => {
// Module: crate::de::error
// Provides: {"impl_236"}
// Dependencies: {}
# [cfg (feature = "parse")] impl < 'i > toml_parser :: ErrorSink for TomlSink < 'i , Vec < Error > > { fn report_error (& mut self , error : toml_parser :: ParseError) { let input = self . input . get_or_insert_with (| | alloc :: sync :: Arc :: from (self . source . input ())) ; let error = Error :: new (input . clone () , error) ; self . sink . push (error) ; } }
};
}
