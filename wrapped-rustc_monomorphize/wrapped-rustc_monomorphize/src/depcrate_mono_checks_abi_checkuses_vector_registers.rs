// Generated macro for uses_vector_registers (function)
macro_rules! Depcrate_mono_checks_abi_checkuses_vector_registers {
() => {
// Module: crate::mono_checks::abi_check
// Provides: {"uses_vector_registers"}
// Dependencies: {}
fn uses_vector_registers (mode : & PassMode , repr : & BackendRepr) -> bool { match mode { PassMode :: Ignore | PassMode :: Indirect { .. } => false , PassMode :: Cast { pad_i32 : _ , cast } => { cast . prefix . iter () . any (| r | r . is_some_and (| x | x . kind == RegKind :: Vector)) || cast . rest . unit . kind == RegKind :: Vector } PassMode :: Direct (..) | PassMode :: Pair (..) => matches ! (repr , BackendRepr :: SimdVector { .. }) , } }
};
}
