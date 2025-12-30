// Generated macro for impl_48 (impl)
macro_rules! Depcrate_test_utilsimpl_48 {
() => {
// Module: crate::test::utils
// Provides: {"impl_48"}
// Dependencies: {}
impl TestFixture { pub fn new () -> TestFixture { let tempdir = tempfile :: Builder :: new () . prefix ("sccache_test") . tempdir () . unwrap () ; let mut builder = std :: fs :: DirBuilder :: new () ; builder . recursive (true) ; let mut paths = vec ! [] ; let mut bins = vec ! [] ; for d in SUBDIRS . iter () { let p = tempdir . path () . join (d) ; builder . create (& p) . unwrap () ; bins . push (mk_bin (& p , BIN_NAME) . unwrap ()) ; paths . push (p) ; } TestFixture { tempdir , paths : env :: join_paths (paths) . unwrap () , bins , } } # [allow (dead_code)] pub fn touch (& self , path : & str) -> io :: Result < PathBuf > { touch (self . tempdir . path () , path) } # [allow (dead_code)] pub fn mk_bin (& self , path : & str) -> io :: Result < PathBuf > { mk_bin (self . tempdir . path () , path) } }
};
}
