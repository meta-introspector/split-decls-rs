// Generated macro for Path (struct)
macro_rules! Depcrate_pathPath {
() => {
// Module: crate::path
// Provides: {"Path"}
// Dependencies: {}
# [doc = " A slice of a path (akin to [`str`])."] # [doc = ""] # [doc = " This type supports a number of operations for inspecting a path, including"] # [doc = " breaking the path into its components (separated by `/` on Unix and by either"] # [doc = " `/` or `\\` on Windows), extracting the file name, determining whether the path"] # [doc = " is absolute, and so on."] # [doc = ""] # [doc = " This is an *unsized* type, meaning that it must always be used behind a"] # [doc = " pointer like `&` or [`Box`]. For an owned version of this type,"] # [doc = " see [`PathBuf`]."] # [doc = ""] # [doc = " More details about the overall approach can be found in"] # [doc = " the [module documentation](self)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Path;"] # [doc = " use std::ffi::OsStr;"] # [doc = ""] # [doc = " // Note: this example does work on Windows"] # [doc = " let path = Path::new(\"./foo/bar.txt\");"] # [doc = ""] # [doc = " let parent = path.parent();"] # [doc = " assert_eq!(parent, Some(Path::new(\"./foo\")));"] # [doc = ""] # [doc = " let file_stem = path.file_stem();"] # [doc = " assert_eq!(file_stem, Some(OsStr::new(\"bar\")));"] # [doc = ""] # [doc = " let extension = path.extension();"] # [doc = " assert_eq!(extension, Some(OsStr::new(\"txt\")));"] # [doc = " ```"] # [cfg_attr (not (test) , rustc_diagnostic_item = "Path")] # [stable (feature = "rust1" , since = "1.0.0")] # [repr (transparent)] pub struct Path { inner : OsStr , }
};
}
