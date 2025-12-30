// Generated macro for Config (struct)
macro_rules! Depcrate_loaderConfig {
() => {
// Module: crate::loader
// Provides: {"Config"}
// Dependencies: {}
# [doc = " [`Handle`]'s configuration."] # [derive (Debug)] pub struct Config { # [doc = " Version number to associate progress updates to the right config"] # [doc = " version."] pub version : u32 , # [doc = " Set of initially loaded files."] pub load : Vec < Entry > , # [doc = " Index of watched entries in `load`."] # [doc = ""] # [doc = " If a path in a watched entry is modified,the [`Handle`] should notify it."] pub watch : Vec < usize > , }
};
}
