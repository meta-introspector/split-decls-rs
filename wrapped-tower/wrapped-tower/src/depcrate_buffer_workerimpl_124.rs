// Generated macro for impl_124 (impl)
macro_rules! Depcrate_buffer_workerimpl_124 {
() => {
// Module: crate::buffer::worker
// Provides: {"impl_124"}
// Dependencies: {}
impl < T , Request > Future for Worker < T , Request > where T : Service < Request > , T :: Error : Into < crate :: BoxError > , { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { if self . finish { return Poll :: Ready (()) ; } loop { match ready ! (self . poll_next_msg (cx)) { Some ((msg , first)) => { let _guard = msg . span . enter () ; if let Some (ref failed) = self . failed { tracing :: trace ! ("notifying caller about worker failure") ; let _ = msg . tx . send (Err (failed . clone ())) ; continue ; } tracing :: trace ! (resumed = ! first , message = "worker received request; waiting for service readiness") ; match self . service . poll_ready (cx) { Poll :: Ready (Ok (())) => { tracing :: debug ! (service . ready = true , message = "processing request") ; let response = self . service . call (msg . request) ; tracing :: trace ! ("returning response future") ; let _ = msg . tx . send (Ok (response)) ; } Poll :: Pending => { tracing :: trace ! (service . ready = false , message = "delay") ; drop (_guard) ; self . current_message = Some (msg) ; return Poll :: Pending ; } Poll :: Ready (Err (e)) => { let error = e . into () ; tracing :: debug ! ({ % error } , "service failed") ; drop (_guard) ; self . failed (error) ; let _ = msg . tx . send (Err (self . failed . as_ref () . expect ("Worker::failed did not set self.failed?") . clone ())) ; } } } None => { self . finish = true ; return Poll :: Ready (()) ; } } } } }
};
}
