// Generated macro for impl_346 (impl)
macro_rules! Depcrate_config_optionsimpl_346 {
() => {
// Module: crate::config::options
// Provides: {"impl_346"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a IgnoreList { type Item = & 'a PathBuf ; type IntoIter = hash_set :: Iter < 'a , PathBuf > ; fn into_iter (self) -> Self :: IntoIter { self . path_set . iter () } }
};
}
