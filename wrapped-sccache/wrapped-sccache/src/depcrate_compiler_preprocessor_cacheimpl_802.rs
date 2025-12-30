// Generated macro for impl_802 (impl)
macro_rules! Depcrate_compiler_preprocessor_cacheimpl_802 {
() => {
// Module: crate::compiler::preprocessor_cache
// Provides: {"impl_802"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Error :: Io (e) => e . fmt (f) , Error :: Deserialization (e) => e . fmt (f) , Error :: UnknownFormat (format) => f . write_fmt (format_args ! ("Unknown preprocessor cache entry format {:x}" , format)) , } } }
};
}
