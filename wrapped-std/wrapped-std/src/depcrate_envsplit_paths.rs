// Generated macro for split_paths (function)
macro_rules! Depcrate_envsplit_paths {
() => {
// Module: crate::env
// Provides: {"split_paths"}
// Dependencies: {}
# [doc = " Parses input according to platform conventions for the `PATH`"] # [doc = " environment variable."] # [doc = ""] # [doc = " Returns an iterator over the paths contained in `unparsed`. The iterator"] # [doc = " element type is [`PathBuf`]."] # [doc = ""] # [doc = " On most Unix platforms, the separator is `:` and on Windows it is `;`. This"] # [doc = " also performs unquoting on Windows."] # [doc = ""] # [doc = " [`join_paths`] can be used to recombine elements."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This will panic on systems where there is no delimited `PATH` variable,"] # [doc = " such as UEFI."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::env;"] # [doc = ""] # [doc = " let key = \"PATH\";"] # [doc = " match env::var_os(key) {"] # [doc = "     Some(paths) => {"] # [doc = "         for path in env::split_paths(&paths) {"] # [doc = "             println!(\"'{}'\", path.display());"] # [doc = "         }"] # [doc = "     }"] # [doc = "     None => println!(\"{key} is not defined in the environment.\")"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "env" , since = "1.0.0")] pub fn split_paths < T : AsRef < OsStr > + ? Sized > (unparsed : & T) -> SplitPaths < '_ > { SplitPaths { inner : os_imp :: split_paths (unparsed . as_ref ()) } }
};
}
