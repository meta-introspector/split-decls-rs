// Generated macro for PcFile (struct)
macro_rules! DepcratePcFile {
() => {
// Module: crate
// Provides: {"PcFile"}
// Dependencies: {}
# [doc = " Parsed knowledge from a .pc file."] # [derive (Debug)] struct PcFile { # [doc = " The pkg-config name of this library."] id : String , # [doc = " List of libraries found as '-l', translated to a given vcpkg_target. e.g. libbrotlicommon.a"] libs : Vec < String > , # [doc = " List of pkgconfig dependencies, e.g. PcFile::id."] deps : Vec < String > , }
};
}
