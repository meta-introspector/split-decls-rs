// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg (feature = "futures-01")] # [cfg_attr (docsrs , doc (cfg (feature = "futures-01")))] impl < T : futures_01 :: Future > futures_01 :: Future for Instrumented < T > { type Item = T :: Item ; type Error = T :: Error ; fn poll (& mut self) -> futures_01 :: Poll < Self :: Item , Self :: Error > { let _enter = self . span . enter () ; self . inner . poll () } }
};
}
