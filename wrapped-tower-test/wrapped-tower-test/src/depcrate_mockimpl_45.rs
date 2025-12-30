// Generated macro for impl_45 (impl)
macro_rules! Depcrate_mockimpl_45 {
() => {
// Module: crate::mock
// Provides: {"impl_45"}
// Dependencies: {}
impl < T , U > Service < T > for Mock < T , U > { type Response = U ; type Error = Error ; type Future = ResponseFuture < U > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let mut state = self . state . lock () . unwrap () ; if state . is_closed { return Poll :: Ready (Err (error :: Closed :: new () . into ())) ; } if let Some (e) = state . err_with . take () { return Poll :: Ready (Err (e)) ; } if self . can_send { return Poll :: Ready (Ok (())) ; } if state . rem > 0 { assert ! (! state . tasks . contains_key (& self . id)) ; self . can_send = true ; Poll :: Ready (Ok (())) } else { * state . tasks . entry (self . id) . or_insert_with (| | cx . waker () . clone ()) = cx . waker () . clone () ; Poll :: Pending } } fn call (& mut self , request : T) -> Self :: Future { let mut state = self . state . lock () . unwrap () ; if state . is_closed { return ResponseFuture :: closed () ; } if ! self . can_send { panic ! ("service not ready; poll_ready must be called first") ; } self . can_send = false ; if state . rem > 0 { state . rem -= 1 ; } let (tx , rx) = oneshot :: channel () ; let send_response = SendResponse { tx } ; match self . tx . lock () . unwrap () . send ((request , send_response)) { Ok (_) => { } Err (_) => { return ResponseFuture :: closed () ; } } ResponseFuture :: new (rx) } }
};
}
