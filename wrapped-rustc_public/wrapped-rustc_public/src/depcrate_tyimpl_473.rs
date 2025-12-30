// Generated macro for impl_473 (impl)
macro_rules! Depcrate_tyimpl_473 {
() => {
// Module: crate::ty
// Provides: {"impl_473"}
// Dependencies: {}
impl IntTy { pub fn num_bytes (self) -> usize { match self { IntTy :: Isize => MachineInfo :: target_pointer_width () . bytes () , IntTy :: I8 => 1 , IntTy :: I16 => 2 , IntTy :: I32 => 4 , IntTy :: I64 => 8 , IntTy :: I128 => 16 , } } }
};
}
