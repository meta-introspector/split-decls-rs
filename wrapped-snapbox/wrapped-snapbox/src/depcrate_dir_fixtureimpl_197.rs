// Generated macro for impl_197 (impl)
macro_rules! Depcrate_dir_fixtureimpl_197 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_197"}
// Dependencies: {}
# [cfg (feature = "dir")] impl DirFixture for & '_ std :: path :: PathBuf { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
};
}
