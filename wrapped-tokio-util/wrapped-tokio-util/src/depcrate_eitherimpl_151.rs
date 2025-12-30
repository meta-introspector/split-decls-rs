// Generated macro for impl_151 (impl)
macro_rules! Depcrate_eitherimpl_151 {
() => {
// Module: crate::either
// Provides: {"impl_151"}
// Dependencies: {}
impl < L , R , O > Future for Either < L , R > where L : Future < Output = O > , R : Future < Output = O > , { type Output = O ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { delegate_call ! (self . poll (cx)) } }
};
}
