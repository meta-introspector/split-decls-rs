// Generated macro for Library (struct)
macro_rules! DepcrateLibrary {
() => {
// Module: crate
// Provides: {"Library"}
// Dependencies: {}
# [doc = " Details of a package that was found"] # [derive (Debug)] pub struct Library { # [doc = " Paths for the linker to search for static or import libraries"] pub link_paths : Vec < PathBuf > , # [doc = " Paths to search at runtme to find DLLs"] pub dll_paths : Vec < PathBuf > , # [doc = " Paths to include files"] pub include_paths : Vec < PathBuf > , # [doc = " cargo: metadata lines"] pub cargo_metadata : Vec < String > , # [doc = " libraries found are static"] pub is_static : bool , # [doc = " DLLs found"] pub found_dlls : Vec < PathBuf > , # [doc = " static libs or import libs found"] pub found_libs : Vec < PathBuf > , # [doc = " link name of libraries found, this is useful to emit linker commands"] pub found_names : Vec < String > , # [doc = " ports that are providing the libraries to link to, in port link order"] pub ports : Vec < String > , # [doc = " the vcpkg triplet that has been selected"] pub vcpkg_triplet : String , }
};
}
