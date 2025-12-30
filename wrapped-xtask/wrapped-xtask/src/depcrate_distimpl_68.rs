// Generated macro for impl_68 (impl)
macro_rules! Depcrate_distimpl_68 {
() => {
// Module: crate::dist
// Provides: {"impl_68"}
// Dependencies: {}
impl Patch { fn new (sh : & Shell , path : impl Into < PathBuf >) -> anyhow :: Result < Patch > { let path = path . into () ; let contents = sh . read_file (& path) ? ; Ok (Patch { path , original_contents : contents . clone () , contents }) } fn replace (& mut self , from : & str , to : & str) -> & mut Patch { assert ! (self . contents . contains (from)) ; self . contents = self . contents . replace (from , to) ; self } fn commit (& self , sh : & Shell) -> anyhow :: Result < () > { sh . write_file (& self . path , & self . contents) ? ; Ok (()) } }
};
}
