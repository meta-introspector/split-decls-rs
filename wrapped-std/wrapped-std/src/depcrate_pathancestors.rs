// Generated macro for Ancestors (struct)
macro_rules! Depcrate_pathAncestors {
() => {
// Module: crate::path
// Provides: {"Ancestors"}
// Dependencies: {}
# [doc = " An iterator over [`Path`] and its ancestors."] # [doc = ""] # [doc = " This `struct` is created by the [`ancestors`] method on [`Path`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = " let path = Path::new(\"/foo/bar\");"] # [doc = ""] # [doc = " for ancestor in path.ancestors() {"] # [doc = "     println!(\"{}\", ancestor.display());"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`ancestors`]: Path::ancestors"] # [derive (Copy , Clone , Debug)] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "path_ancestors" , since = "1.28.0")] pub struct Ancestors < 'a > { next : Option < & 'a Path > , }
};
}
