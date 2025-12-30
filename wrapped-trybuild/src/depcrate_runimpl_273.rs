// Generated macro for impl_273 (impl)
macro_rules! Depcrate_runimpl_273 {
() => {
// Module: crate::run
// Provides: {"impl_273"}
// Dependencies: {}
impl ExpandedTest { fn run (self , project : & Project) -> Result < Outcome > { match self . error { None => self . test . run (project , & self . name) , Some (error) => { let show_expected = false ; message :: begin_test (& self . test , show_expected) ; Err (error) } } } }
};
}
