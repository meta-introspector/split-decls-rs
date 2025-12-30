// Generated macro for impl_2911 (impl)
macro_rules! Depcrate_pathimpl_2911 {
() => {
// Module: crate::path
// Provides: {"impl_2911"}
// Dependencies: {}
# [stable (feature = "path_into_iter" , since = "1.6.0")] impl < 'a > IntoIterator for & 'a PathBuf { type Item = & 'a OsStr ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Iter < 'a > { self . iter () } }
};
}
