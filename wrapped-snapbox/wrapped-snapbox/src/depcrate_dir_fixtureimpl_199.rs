// Generated macro for impl_199 (impl)
macro_rules! Depcrate_dir_fixtureimpl_199 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_199"}
// Dependencies: {}
# [cfg (feature = "dir")] impl DirFixture for str { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { std :: path :: Path :: new (self) . write_to_path (root) } }
};
}
