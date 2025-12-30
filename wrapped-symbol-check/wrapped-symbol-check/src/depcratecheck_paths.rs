// Generated macro for check_paths (function)
macro_rules! Depcratecheck_paths {
() => {
// Module: crate
// Provides: {"check_paths"}
// Dependencies: {}
fn check_paths < P : AsRef < Path > > (paths : & [P]) { for path in paths { let path = path . as_ref () ; println ! ("Checking {}" , path . display ()) ; let archive = BinFile :: from_path (path) ; verify_no_duplicates (& archive) ; verify_core_symbols (& archive) ; } }
};
}
