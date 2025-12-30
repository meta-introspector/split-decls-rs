// Generated macro for SplitPaths (struct)
macro_rules! Depcrate_envSplitPaths {
() => {
// Module: crate::env
// Provides: {"SplitPaths"}
// Dependencies: {}
# [doc = " An iterator that splits an environment variable into paths according to"] # [doc = " platform-specific conventions."] # [doc = ""] # [doc = " The iterator element type is [`PathBuf`]."] # [doc = ""] # [doc = " This structure is created by [`env::split_paths()`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`env::split_paths()`]: split_paths"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "env" , since = "1.0.0")] pub struct SplitPaths < 'a > { inner : os_imp :: SplitPaths < 'a > , }
};
}
