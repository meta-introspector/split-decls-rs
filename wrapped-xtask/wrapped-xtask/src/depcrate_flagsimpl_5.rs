// Generated macro for impl_5 (impl)
macro_rules! Depcrate_flagsimpl_5 {
() => {
// Module: crate::flags
// Provides: {"impl_5"}
// Dependencies: {}
impl FromStr for PgoTrainingCrate { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "rust-analyzer" => Ok (Self :: RustAnalyzer) , url => Ok (Self :: GitHub (url . to_owned ())) , } } }
};
}
