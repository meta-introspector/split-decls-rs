// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg (feature = "std-future")] # [cfg_attr (docsrs , doc (cfg (feature = "std-future")))] impl < T : crate :: stdlib :: future :: Future > crate :: stdlib :: future :: Future for Instrumented < T > { type Output = T :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> core :: task :: Poll < Self :: Output > { let (span , inner) = self . project () . span_and_inner_pin_mut () ; let _enter = span . enter () ; inner . poll (cx) } }
};
}
