// Generated macro for Abi (trait)
macro_rules! Depcrate_inherentAbi {
() => {
// Module: crate::inherent
// Provides: {"Abi"}
// Dependencies: {}
pub trait Abi < I : Interner < Abi = Self > > : Copy + Debug + Hash + Eq { fn rust () -> Self ; # [doc = " Whether this ABI is `extern \"Rust\"`."] fn is_rust (self) -> bool ; }
};
}
