// Generated macro for impl_494 (impl)
macro_rules! Depcrate_compiler_clangimpl_494 {
() => {
// Module: crate::compiler::clang
// Provides: {"impl_494"}
// Dependencies: {}
impl Clang { fn is_minversion (& self , major : u64) -> bool { if self . is_appleclang { return false ; } let version_val = match self . version . clone () { Some (version_val) => version_val , None => return false , } ; let version_str = match version_val . split (' ') . find (| x | x . contains ('.')) { Some (version_str) => version_str , None => return false , } ; let parsed_version = match Version :: parse (version_str . trim_end_matches ('"')) { Ok (parsed_version) => parsed_version , Err (e) => return false , } ; parsed_version >= (Version { major , minor : 0 , patch : 0 , pre : Prerelease :: default () , build : BuildMetadata :: default () , }) } }
};
}
