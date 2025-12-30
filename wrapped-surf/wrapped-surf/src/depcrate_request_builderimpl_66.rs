// Generated macro for impl_66 (impl)
macro_rules! Depcrate_request_builderimpl_66 {
() => {
// Module: crate::request_builder
// Provides: {"impl_66"}
// Dependencies: {}
impl Future for RequestBuilder { type Output = Result < Response > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { if self . fut . is_none () { let req = self . req . take () . unwrap () ; let client = self . client . take () . unwrap_or_else (Client :: new_shared_or_panic) ; self . fut = Some (Box :: pin (async move { client . send (req) . await })) } self . fut . as_mut () . unwrap () . as_mut () . poll (cx) } }
};
}
