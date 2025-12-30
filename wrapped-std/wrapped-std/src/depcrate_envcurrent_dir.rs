// Generated macro for current_dir (function)
macro_rules! Depcrate_envcurrent_dir {
() => {
// Module: crate::env
// Provides: {"current_dir"}
// Dependencies: {}
# [doc = " Returns the current working directory as a [`PathBuf`]."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " This function [currently] corresponds to the `getcwd` function on Unix"] # [doc = " and the `GetCurrentDirectoryW` function on Windows."] # [doc = ""] # [doc = " [currently]: crate::io#platform-specific-behavior"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an [`Err`] if the current working directory value is invalid."] # [doc = " Possible cases:"] # [doc = ""] # [doc = " * Current directory does not exist."] # [doc = " * There are insufficient permissions to access the current directory."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::env;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let path = env::current_dir()?;"] # [doc = "     println!(\"The current directory is {}\", path.display());"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc (alias = "pwd")] # [doc (alias = "getcwd")] # [doc (alias = "GetCurrentDirectory")] # [stable (feature = "env" , since = "1.0.0")] pub fn current_dir () -> io :: Result < PathBuf > { os_imp :: getcwd () }
};
}
