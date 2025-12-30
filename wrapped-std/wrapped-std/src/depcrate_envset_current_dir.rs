// Generated macro for set_current_dir (function)
macro_rules! Depcrate_envset_current_dir {
() => {
// Module: crate::env
// Provides: {"set_current_dir"}
// Dependencies: {}
# [doc = " Changes the current working directory to the specified path."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function [currently] corresponds to the `chdir` function on Unix"] # [doc = " and the `SetCurrentDirectoryW` function on Windows."] # [doc = ""] # [doc = " Returns an [`Err`] if the operation fails."] # [doc = ""] # [doc = " [currently]: crate::io#platform-specific-behavior"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::env;"] # [doc = " use std::path::Path;"] # [doc = ""] # [doc = " let root = Path::new(\"/\");"] # [doc = " assert!(env::set_current_dir(&root).is_ok());"] # [doc = " println!(\"Successfully changed working directory to {}!\", root.display());"] # [doc = " ```"] # [doc (alias = "chdir" , alias = "SetCurrentDirectory" , alias = "SetCurrentDirectoryW")] # [stable (feature = "env" , since = "1.0.0")] pub fn set_current_dir < P : AsRef < Path > > (path : P) -> io :: Result < () > { os_imp :: chdir (path . as_ref ()) }
};
}
