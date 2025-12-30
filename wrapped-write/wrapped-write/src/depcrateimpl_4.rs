// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
# [cfg (feature = "std")] impl uWrite for String { type Error = Infallible ; fn write_str (& mut self , s : & str) -> Result < () , Infallible > { self . push_str (s) ; Ok (()) } }
};
}
