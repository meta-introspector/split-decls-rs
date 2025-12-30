// Generated macro for impl_475 (impl)
macro_rules! Depcrate_tyimpl_475 {
() => {
// Module: crate::ty
// Provides: {"impl_475"}
// Dependencies: {}
impl UintTy { pub fn num_bytes (self) -> usize { match self { UintTy :: Usize => MachineInfo :: target_pointer_width () . bytes () , UintTy :: U8 => 1 , UintTy :: U16 => 2 , UintTy :: U32 => 4 , UintTy :: U64 => 8 , UintTy :: U128 => 16 , } } }
};
}
