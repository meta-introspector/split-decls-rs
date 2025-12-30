// Generated macro for impl_65 (impl)
macro_rules! Depcrate_abiimpl_65 {
() => {
// Module: crate::abi
// Provides: {"impl_65"}
// Dependencies: {}
impl Scalar { pub fn has_niche (& self , target : & MachineInfo) -> bool { match self { Scalar :: Initialized { value , valid_range } => { ! valid_range . is_full (value . size (target)) . unwrap () } Scalar :: Union { .. } => false , } } }
};
}
