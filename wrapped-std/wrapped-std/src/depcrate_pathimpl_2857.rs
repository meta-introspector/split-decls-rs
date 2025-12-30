// Generated macro for impl_2857 (impl)
macro_rules! Depcrate_pathimpl_2857 {
() => {
// Module: crate::path
// Provides: {"impl_2857"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < P : AsRef < Path > > FromIterator < P > for PathBuf { # [doc = " Creates a new `PathBuf` from the [`Path`] elements of an iterator."] # [doc = ""] # [doc = " This uses [`push`](Self::push) to add each element, so can be used to adjoin multiple path"] # [doc = " [components](Components)."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use std::path::PathBuf;"] # [doc = " let path = PathBuf::from_iter([\"/tmp\", \"foo\", \"bar\"]);"] # [doc = " assert_eq!(path, PathBuf::from(\"/tmp/foo/bar\"));"] # [doc = " ```"] # [doc = ""] # [doc = " See documentation for [`push`](Self::push) for more details on how the path is constructed."] fn from_iter < I : IntoIterator < Item = P > > (iter : I) -> PathBuf { let mut buf = PathBuf :: new () ; buf . extend (iter) ; buf } }
};
}
