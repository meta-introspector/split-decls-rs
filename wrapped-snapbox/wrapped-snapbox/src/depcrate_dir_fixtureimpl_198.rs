// Generated macro for impl_198 (impl)
macro_rules! Depcrate_dir_fixtureimpl_198 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_198"}
// Dependencies: {}
# [cfg (feature = "dir")] impl DirFixture for std :: path :: PathBuf { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
};
}
