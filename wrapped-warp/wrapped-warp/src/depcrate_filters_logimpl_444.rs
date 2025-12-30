// Generated macro for impl_444 (impl)
macro_rules! Depcrate_filters_logimpl_444 {
() => {
// Module: crate::filters::log
// Provides: {"impl_444"}
// Dependencies: {}
impl < FN , F > WrapSealed < F > for Log < FN > where FN : Fn (Info < '_ >) + Clone + Send , F : Filter + Clone + Send , F :: Extract : Reply , F :: Error : IsReject , { type Wrapped = WithLog < FN , F > ; fn wrap (& self , filter : F) -> Self :: Wrapped { WithLog { filter , log : self . clone () , } } }
};
}
