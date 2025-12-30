// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl ChildExt for Child { fn wait_timeout (& mut self , dur : Duration) -> io :: Result < Option < ExitStatus > > { drop (self . stdin . take ()) ; imp :: wait_timeout (self , dur) } }
};
}
