// Generated macro for is_loongarch_aggregate (function)
macro_rules! Depcrate_callconv_loongarchis_loongarch_aggregate {
() => {
// Module: crate::callconv::loongarch
// Provides: {"is_loongarch_aggregate"}
// Dependencies: {}
fn is_loongarch_aggregate < Ty > (arg : & ArgAbi < '_ , Ty >) -> bool { match arg . layout . backend_repr { BackendRepr :: SimdVector { .. } => true , _ => arg . layout . is_aggregate () , } }
};
}
