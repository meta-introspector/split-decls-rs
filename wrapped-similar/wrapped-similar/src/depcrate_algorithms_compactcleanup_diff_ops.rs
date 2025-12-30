// Generated macro for cleanup_diff_ops (function)
macro_rules! Depcrate_algorithms_compactcleanup_diff_ops {
() => {
// Module: crate::algorithms::compact
// Provides: {"cleanup_diff_ops"}
// Dependencies: {}
pub fn cleanup_diff_ops < Old , New > (old : & Old , new : & New , ops : & mut Vec < DiffOp >) where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , New :: Output : PartialEq < Old :: Output > , { let mut pointer = 0 ; while let Some (& op) = ops . get (pointer) { if let DiffTag :: Delete = op . tag () { pointer = shift_diff_ops_up (ops , old , new , pointer) ; pointer = shift_diff_ops_down (ops , old , new , pointer) ; } pointer += 1 ; } let mut pointer = 0 ; while let Some (& op) = ops . get (pointer) { if let DiffTag :: Insert = op . tag () { pointer = shift_diff_ops_up (ops , old , new , pointer) ; pointer = shift_diff_ops_down (ops , old , new , pointer) ; } pointer += 1 ; } }
};
}
