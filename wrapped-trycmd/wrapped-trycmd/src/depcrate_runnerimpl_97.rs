// Generated macro for impl_97 (impl)
macro_rules! Depcrate_runnerimpl_97 {
() => {
// Module: crate::runner
// Provides: {"impl_97"}
// Dependencies: {}
impl Mode { pub (crate) fn initialize (& self) -> Result < () , std :: io :: Error > { match self { Self :: Fail => { } Self :: Overwrite => { } Self :: Dump (root) => { std :: fs :: create_dir_all (root) ? ; let gitignore_path = root . join (".gitignore") ; std :: fs :: write (gitignore_path , "*\n") ? ; } } Ok (()) } }
};
}
