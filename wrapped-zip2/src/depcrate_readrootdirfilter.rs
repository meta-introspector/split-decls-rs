// Generated macro for RootDirFilter (trait)
macro_rules! Depcrate_readRootDirFilter {
() => {
// Module: crate::read
// Provides: {"RootDirFilter"}
// Dependencies: {}
# [doc = " A filter that determines whether an entry should be ignored when searching"] # [doc = " for the root directory of a Zip archive."] # [doc = ""] # [doc = " Returns `true` if the entry should be considered, and `false` if it should"] # [doc = " be ignored."] # [doc = ""] # [doc = " See [`root_dir_common_filter`] for a sensible default filter."] pub trait RootDirFilter : Fn (& Path) -> bool { }
};
}
