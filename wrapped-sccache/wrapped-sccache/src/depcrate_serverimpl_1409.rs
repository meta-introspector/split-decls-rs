// Generated macro for impl_1409 (impl)
macro_rules! Depcrate_serverimpl_1409 {
() => {
// Module: crate::server
// Provides: {"impl_1409"}
// Dependencies: {}
impl < C > Service < SccacheRequest > for Arc < SccacheService < C > > where C : CommandCreatorSync + Send + Sync + 'static , { type Response = SccacheResponse ; type Error = Error ; type Future = Pin < Box < dyn Future < Output = Result < Self :: Response > > + Send + 'static > > ; fn call (& mut self , req : SccacheRequest) -> Self :: Future { trace ! ("handle_client") ; drop (self . tx . clone () . start_send (ServerMessage :: Request)) ; let me = self . clone () ; Box :: pin (async move { match req . into_inner () { Request :: Compile (compile) => { debug ! ("handle_client: compile") ; me . stats . lock () . await . compile_requests += 1 ; me . handle_compile (compile) . await } Request :: GetStats => { debug ! ("handle_client: get_stats") ; me . get_info () . await . map (| i | Response :: Stats (Box :: new (i))) . map (Message :: WithoutBody) } Request :: DistStatus => { debug ! ("handle_client: dist_status") ; me . get_dist_status () . await . map (Response :: DistStatus) . map (Message :: WithoutBody) } Request :: ZeroStats => { debug ! ("handle_client: zero_stats") ; me . zero_stats () . await ; Ok (Message :: WithoutBody (Response :: ZeroStats)) } Request :: Shutdown => { debug ! ("handle_client: shutdown") ; let mut tx = me . tx . clone () ; future :: try_join (async { let _ = tx . send (ServerMessage :: Shutdown) . await ; Ok (()) } , me . get_info () ,) . await . map (move | (_ , info) | { Message :: WithoutBody (Response :: ShuttingDown (Box :: new (info))) }) } } }) } fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () > > { Poll :: Ready (Ok (())) } }
};
}
