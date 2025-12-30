// Generated macro for impl_2912 (impl)
macro_rules! Depcrate_pathimpl_2912 {
() => {
// Module: crate::path
// Provides: {"impl_2912"}
// Dependencies: {}
# [stable (feature = "path_into_iter" , since = "1.6.0")] impl < 'a > IntoIterator for & 'a Path { type Item = & 'a OsStr ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Iter < 'a > { self . iter () } }
};
}
