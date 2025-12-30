// Generated macro for Display (struct)
macro_rules! Depcrate_pathDisplay {
() => {
// Module: crate::path
// Provides: {"Display"}
// Dependencies: {}
# [doc = " Helper struct for safely printing paths with [`format!`] and `{}`."] # [doc = ""] # [doc = " A [`Path`] might contain non-Unicode data. This `struct` implements the"] # [doc = " [`Display`] trait in a way that mitigates that. It is created by the"] # [doc = " [`display`](Path::display) method on [`Path`]. This may perform lossy"] # [doc = " conversion, depending on the platform. If you would like an implementation"] # [doc = " which escapes the path please use [`Debug`] instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = " let path = Path::new(\"/tmp/foo.rs\");"] # [doc = ""] # [doc = " println!(\"{}\", path.display());"] # [doc = " ```"] # [doc = ""] # [doc = " [`Display`]: fmt::Display"] # [doc = " [`format!`]: crate::format"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Display < 'a > { inner : os_str :: Display < 'a > , }
};
}
