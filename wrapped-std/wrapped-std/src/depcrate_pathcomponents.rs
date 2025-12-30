// Generated macro for Components (struct)
macro_rules! Depcrate_pathComponents {
() => {
// Module: crate::path
// Provides: {"Components"}
// Dependencies: {}
# [doc = " An iterator over the [`Component`]s of a [`Path`]."] # [doc = ""] # [doc = " This `struct` is created by the [`components`] method on [`Path`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = " let path = Path::new(\"/tmp/foo/bar.txt\");"] # [doc = ""] # [doc = " for component in path.components() {"] # [doc = "     println!(\"{component:?}\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`components`]: Path::components"] # [derive (Clone)] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Components < 'a > { path : & 'a [u8] , prefix : Option < Prefix < 'a > > , has_physical_root : bool , front : State , back : State , }
};
}
