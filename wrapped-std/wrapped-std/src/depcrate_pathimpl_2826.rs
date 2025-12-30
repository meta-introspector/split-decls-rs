// Generated macro for impl_2826 (impl)
macro_rules! Depcrate_pathimpl_2826 {
() => {
// Module: crate::path
// Provides: {"impl_2826"}
// Dependencies: {}
impl < 'a > Iter < 'a > { # [doc = " Extracts a slice corresponding to the portion of the path remaining for iteration."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = " let mut iter = Path::new(\"/tmp/foo/bar.txt\").iter();"] # [doc = " iter.next();"] # [doc = " iter.next();"] # [doc = ""] # [doc = " assert_eq!(Path::new(\"foo/bar.txt\"), iter.as_path());"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [must_use] # [inline] pub fn as_path (& self) -> & 'a Path { self . inner . as_path () } }
};
}
