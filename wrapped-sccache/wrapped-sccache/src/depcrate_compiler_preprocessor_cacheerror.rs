// Generated macro for Error (enum)
macro_rules! Depcrate_compiler_preprocessor_cacheError {
() => {
// Module: crate::compiler::preprocessor_cache
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] pub enum Error { Io (std :: io :: Error) , Deserialization (bincode :: Error) , UnknownFormat (u8) , }
};
}
