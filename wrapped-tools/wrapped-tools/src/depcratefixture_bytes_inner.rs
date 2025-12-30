// Generated macro for fixture_bytes_inner (function)
macro_rules! Depcratefixture_bytes_inner {
() => {
// Module: crate
// Provides: {"fixture_bytes_inner"}
// Dependencies: {}
fn fixture_bytes_inner (path : impl AsRef < Path > , root : DirectoryRoot) -> Vec < u8 > { match std :: fs :: read (fixture_path_inner (path . as_ref () , root)) { Ok (res) => res , Err (_) => panic ! ("File at '{}' not found" , path . as_ref () . display ()) , } }
};
}
