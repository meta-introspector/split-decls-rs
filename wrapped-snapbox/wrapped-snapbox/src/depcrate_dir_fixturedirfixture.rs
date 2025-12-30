// Generated macro for DirFixture (trait)
macro_rules! Depcrate_dir_fixtureDirFixture {
() => {
// Module: crate::dir::fixture
// Provides: {"DirFixture"}
// Dependencies: {}
# [doc = " Collection of files"] pub trait DirFixture : std :: fmt :: Debug { # [doc = " Initialize a test fixture directory `root`"] fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > ; }
};
}
