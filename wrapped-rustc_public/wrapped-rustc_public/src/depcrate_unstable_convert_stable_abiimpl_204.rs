// Generated macro for impl_204 (impl)
macro_rules! Depcrate_unstable_convert_stable_abiimpl_204 {
() => {
// Module: crate::unstable::convert::stable::abi
// Provides: {"impl_204"}
// Dependencies: {}
impl < 'tcx > Stable < 'tcx > for rustc_abi :: Integer { type T = IntegerLength ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { rustc_abi :: Integer :: I8 => IntegerLength :: I8 , rustc_abi :: Integer :: I16 => IntegerLength :: I16 , rustc_abi :: Integer :: I32 => IntegerLength :: I32 , rustc_abi :: Integer :: I64 => IntegerLength :: I64 , rustc_abi :: Integer :: I128 => IntegerLength :: I128 , } } }
};
}
