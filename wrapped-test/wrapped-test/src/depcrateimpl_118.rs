// Generated macro for impl_118 (impl)
macro_rules! Depcrateimpl_118 {
() => {
// Module: crate
// Provides: {"impl_118"}
// Dependencies: {}
impl Opts { pub fn run (& self , wit_bindgen : & Path) -> Result < () > { Runner { opts : self , rust_state : None , wit_bindgen , test_runner : runner :: TestRunner :: new (& self . runner) ? , } . run () } }
};
}
