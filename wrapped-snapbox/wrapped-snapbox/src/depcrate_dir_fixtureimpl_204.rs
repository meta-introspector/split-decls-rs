// Generated macro for impl_204 (impl)
macro_rules! Depcrate_dir_fixtureimpl_204 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_204"}
// Dependencies: {}
impl < const N : usize , P , S > DirFixture for [(P , S) ; N] where P : AsRef < std :: path :: Path > , P : std :: fmt :: Debug , S : AsRef < [u8] > , S : std :: fmt :: Debug , { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { let s : & [(P , S)] = self ; s . write_to_path (root) } }
};
}
