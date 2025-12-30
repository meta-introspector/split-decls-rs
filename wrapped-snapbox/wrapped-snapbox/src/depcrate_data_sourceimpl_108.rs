// Generated macro for impl_108 (impl)
macro_rules! Depcrate_data_sourceimpl_108 {
() => {
// Module: crate::data::source
// Provides: {"impl_108"}
// Dependencies: {}
impl DataSource { pub fn path (path : impl Into < std :: path :: PathBuf >) -> Self { Self { inner : DataSourceInner :: Path (path . into ()) , } } pub fn is_path (& self) -> bool { self . as_path () . is_some () } pub fn as_path (& self) -> Option < & std :: path :: Path > { match & self . inner { DataSourceInner :: Path (value) => Some (value . as_ref ()) , _ => None , } } pub fn is_inline (& self) -> bool { self . as_inline () . is_some () } pub fn as_inline (& self) -> Option < & Inline > { match & self . inner { DataSourceInner :: Inline (value) => Some (value) , _ => None , } } }
};
}
