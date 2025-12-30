// Generated macro for impl_112 (impl)
macro_rules! Depcrate_data_sourceimpl_112 {
() => {
// Module: crate::data::source
// Provides: {"impl_112"}
// Dependencies: {}
impl std :: fmt :: Display for DataSource { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . inner { DataSourceInner :: Path (value) => crate :: dir :: display_relpath (value) . fmt (f) , DataSourceInner :: Inline (value) => value . fmt (f) , } } }
};
}
