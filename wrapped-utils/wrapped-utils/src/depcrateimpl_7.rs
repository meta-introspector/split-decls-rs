// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < W > uWrite for Ignore < W > where W : uWrite , { type Error = Infallible ; fn write_str (& mut self , s : & str) -> Result < () , Infallible > { let _ = self . writer . write_str (s) ; Ok (()) } }
};
}
