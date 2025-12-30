// Generated macro for find_package (function)
macro_rules! Depcratefind_package {
() => {
// Module: crate
// Provides: {"find_package"}
// Dependencies: {}
# [doc = " Find the package `package` in a Vcpkg tree."] # [doc = ""] # [doc = " Emits cargo metadata to link to libraries provided by the Vcpkg package/port"] # [doc = " named, and any (non-system) libraries that they depend on."] # [doc = ""] # [doc = " This will select the architecture and linkage based on environment"] # [doc = " variables and build flags as described in the module docs."] pub fn find_package (package : & str) -> Result < Library , Error > { Config :: new () . find_package (package) }
};
}
