// Generated macro for Iter (struct)
macro_rules! Depcrate_pathIter {
() => {
// Module: crate::path
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the [`Component`]s of a [`Path`], as [`OsStr`] slices."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`Path`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter`]: Path::iter"] # [derive (Clone)] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Iter < 'a > { inner : Components < 'a > , }
};
}
