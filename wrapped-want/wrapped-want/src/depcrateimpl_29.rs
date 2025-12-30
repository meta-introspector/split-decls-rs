// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl Future for Want < '_ > { type Output = Result < () , Closed > ; fn poll (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { self . 0 . poll_want (cx) } }
};
}
