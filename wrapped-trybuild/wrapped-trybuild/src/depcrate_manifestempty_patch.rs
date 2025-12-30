// Generated macro for empty_patch (function)
macro_rules! Depcrate_manifestempty_patch {
() => {
// Module: crate::manifest
// Provides: {"empty_patch"}
// Dependencies: {}
fn empty_patch (patch : & Map < String , RegistryPatch >) -> bool { patch . values () . all (| registry_patch | registry_patch . crates . is_empty ()) }
};
}
