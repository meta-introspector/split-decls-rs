// Generated macro for impl_18 (impl)
macro_rules! Depcrate_balance_p2c_layerimpl_18 {
() => {
// Module: crate::balance::p2c::layer
// Provides: {"impl_18"}
// Dependencies: {}
impl < S , Req > Layer < S > for MakeBalanceLayer < S , Req > { type Service = MakeBalance < S , Req > ; fn layer (& self , make_discover : S) -> Self :: Service { MakeBalance :: new (make_discover) } }
};
}
