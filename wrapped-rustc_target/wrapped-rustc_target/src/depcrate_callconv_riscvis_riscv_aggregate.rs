// Generated macro for is_riscv_aggregate (function)
macro_rules! Depcrate_callconv_riscvis_riscv_aggregate {
() => {
// Module: crate::callconv::riscv
// Provides: {"is_riscv_aggregate"}
// Dependencies: {}
fn is_riscv_aggregate < Ty > (arg : & ArgAbi < '_ , Ty >) -> bool { match arg . layout . backend_repr { BackendRepr :: SimdVector { .. } => true , _ => arg . layout . is_aggregate () , } }
};
}
