// Generated macro for impl_202 (impl)
macro_rules! Depcrate_dir_fixtureimpl_202 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_202"}
// Dependencies: {}
# [cfg (feature = "dir")] impl DirFixture for String { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
};
}
