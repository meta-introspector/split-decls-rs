// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Future for Timeout { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < () > { Pin :: new (& mut self . inner) . poll (cx) . map (| _ | ()) } }
};
}
