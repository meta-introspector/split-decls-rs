// Generated macro for impl_196 (impl)
macro_rules! Depcrate_dir_fixtureimpl_196 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_196"}
// Dependencies: {}
# [cfg (feature = "dir")] impl DirFixture for & '_ std :: path :: Path { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
};
}
