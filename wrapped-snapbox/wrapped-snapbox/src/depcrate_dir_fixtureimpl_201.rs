// Generated macro for impl_201 (impl)
macro_rules! Depcrate_dir_fixtureimpl_201 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_201"}
// Dependencies: {}
# [cfg (feature = "dir")] impl DirFixture for & '_ String { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
};
}
