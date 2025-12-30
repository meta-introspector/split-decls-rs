// Generated macro for impl_200 (impl)
macro_rules! Depcrate_dir_fixtureimpl_200 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_200"}
// Dependencies: {}
# [cfg (feature = "dir")] impl DirFixture for & '_ str { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
};
}
