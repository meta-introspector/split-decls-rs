// Generated macro for is_xtensa_aggregate (function)
macro_rules! Depcrate_callconv_xtensais_xtensa_aggregate {
() => {
// Module: crate::callconv::xtensa
// Provides: {"is_xtensa_aggregate"}
// Dependencies: {}
fn is_xtensa_aggregate < 'a , Ty > (arg : & ArgAbi < 'a , Ty >) -> bool { match arg . layout . backend_repr { BackendRepr :: SimdVector { .. } => true , _ => arg . layout . is_aggregate () , } }
};
}
