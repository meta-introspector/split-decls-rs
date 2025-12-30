// Generated macro for Display (struct)
macro_rules! Depcrate_ffi_os_strDisplay {
() => {
// Module: crate::ffi::os_str
// Provides: {"Display"}
// Dependencies: {}
# [doc = " Helper struct for safely printing an [`OsStr`] with [`format!`] and `{}`."] # [doc = ""] # [doc = " An [`OsStr`] might contain non-Unicode data. This `struct` implements the"] # [doc = " [`Display`] trait in a way that mitigates that. It is created by the"] # [doc = " [`display`](OsStr::display) method on [`OsStr`]. This may perform lossy"] # [doc = " conversion, depending on the platform. If you would like an implementation"] # [doc = " which escapes the [`OsStr`] please use [`Debug`] instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::OsStr;"] # [doc = ""] # [doc = " let s = OsStr::new(\"Hello, world!\");"] # [doc = " println!(\"{}\", s.display());"] # [doc = " ```"] # [doc = ""] # [doc = " [`Display`]: fmt::Display"] # [doc = " [`format!`]: crate::format"] # [stable (feature = "os_str_display" , since = "1.87.0")] pub struct Display < 'a > { os_str : & 'a OsStr , }
};
}
