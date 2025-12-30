// Generated macro for impl_56 (impl)
macro_rules! Depcrate_tests_utilimpl_56 {
() => {
// Module: crate::tests::util
// Provides: {"impl_56"}
// Dependencies: {}
impl TempDir { # [doc = " Create a new empty temporary directory under the system's configured"] # [doc = " temporary directory."] pub fn new () -> Result < TempDir > { # [allow (deprecated)] use std :: sync :: atomic :: { AtomicUsize , Ordering , ATOMIC_USIZE_INIT } ; static TRIES : usize = 100 ; # [allow (deprecated)] static COUNTER : AtomicUsize = ATOMIC_USIZE_INIT ; let tmpdir = env :: temp_dir () ; for _ in 0 .. TRIES { let count = COUNTER . fetch_add (1 , Ordering :: SeqCst) ; let path = tmpdir . join ("rust-walkdir") . join (count . to_string ()) ; if path . is_dir () { continue ; } fs :: create_dir_all (& path) . map_err (| e | { err ! ("failed to create {}: {}" , path . display () , e) }) ? ; return Ok (TempDir (path)) ; } Err (err ! ("failed to create temp dir after {} tries" , TRIES)) } # [doc = " Return the underlying path to this temporary directory."] pub fn path (& self) -> & Path { & self . 0 } }
};
}
