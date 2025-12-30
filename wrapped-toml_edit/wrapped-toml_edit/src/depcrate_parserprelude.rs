// Generated macro for prelude (module)
macro_rules! Depcrate_parserprelude {
() => {
// Module: crate::parser
// Provides: {"prelude"}
// Dependencies: {}
pub (crate) mod prelude { pub (crate) use toml_parser :: ErrorSink ; pub (crate) use toml_parser :: ParseError ; pub (crate) use toml_parser :: parser :: EventKind ; pub (crate) use winnow :: stream :: Stream as _ ; pub (crate) type Input < 'i > = winnow :: stream :: TokenSlice < 'i , toml_parser :: parser :: Event > ; # [cfg (feature = "debug")] pub (crate) use super :: debug :: TraceScope ; # [cfg (feature = "debug")] pub (crate) use super :: debug :: trace ; }
};
}
