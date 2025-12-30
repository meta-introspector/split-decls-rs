// Generated macro for impl_2816 (impl)
macro_rules! Depcrate_pathimpl_2816 {
() => {
// Module: crate::path
// Provides: {"impl_2816"}
// Dependencies: {}
impl < 'a > Component < 'a > { # [doc = " Extracts the underlying [`OsStr`] slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = " let path = Path::new(\"./tmp/foo/bar.txt\");"] # [doc = " let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();"] # [doc = " assert_eq!(&components, &[\".\", \"tmp\", \"foo\", \"bar.txt\"]);"] # [doc = " ```"] # [must_use = "`self` will be dropped if the result is not used"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn as_os_str (self) -> & 'a OsStr { match self { Component :: Prefix (p) => p . as_os_str () , Component :: RootDir => OsStr :: new (MAIN_SEP_STR) , Component :: CurDir => OsStr :: new (".") , Component :: ParentDir => OsStr :: new ("..") , Component :: Normal (path) => path , } } }
};
}
