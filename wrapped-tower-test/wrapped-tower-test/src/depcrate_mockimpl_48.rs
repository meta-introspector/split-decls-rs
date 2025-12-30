// Generated macro for impl_48 (impl)
macro_rules! Depcrate_mockimpl_48 {
() => {
// Module: crate::mock
// Provides: {"impl_48"}
// Dependencies: {}
impl < T , U > Handle < T , U > { # [doc = " Asynchronously gets the next request"] pub fn poll_request (& mut self) -> Poll < Option < Request < T , U > > > { tokio_test :: task :: spawn (()) . enter (| cx , _ | Box :: pin (self . rx . recv ()) . as_mut () . poll (cx)) } # [doc = " Gets the next request."] pub async fn next_request (& mut self) -> Option < Request < T , U > > { self . rx . recv () . await } # [doc = " Allow a certain number of requests"] pub fn allow (& mut self , num : u64) { let mut state = self . state . lock () . unwrap () ; state . rem = num ; if num > 0 { for (_ , task) in state . tasks . drain () { task . wake () ; } } } # [doc = " Make the next poll_ method error with the given error."] pub fn send_error < E : Into < Error > > (& mut self , e : E) { let mut state = self . state . lock () . unwrap () ; state . err_with = Some (e . into ()) ; for (_ , task) in state . tasks . drain () { task . wake () ; } } }
};
}
