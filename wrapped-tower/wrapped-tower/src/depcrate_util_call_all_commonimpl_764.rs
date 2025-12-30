// Generated macro for impl_764 (impl)
macro_rules! Depcrate_util_call_all_commonimpl_764 {
() => {
// Module: crate::util::call_all::common
// Provides: {"impl_764"}
// Dependencies: {}
impl < Svc , S , Q > Stream for CallAll < Svc , S , Q > where Svc : Service < S :: Item > , S : Stream , Q : Drive < Svc :: Future > , { type Item = Result < Svc :: Response , Svc :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { if let Poll :: Ready (r) = this . queue . poll (cx) { if let Some (rsp) = r . transpose () ? { return Poll :: Ready (Some (Ok (rsp))) ; } } if * this . eof { if this . queue . is_empty () { return Poll :: Ready (None) ; } else { return Poll :: Pending ; } } if this . curr_req . is_none () { * this . curr_req = match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (next_req) => Some (next_req) , None => { * this . eof = true ; continue ; } } ; } let svc = this . service . as_mut () . expect ("Using CallAll after extracting inner Service") ; if let Err (e) = ready ! (svc . poll_ready (cx)) { * this . eof = true ; return Poll :: Ready (Some (Err (e))) ; } this . queue . push (svc . call (this . curr_req . take () . unwrap ())) ; } } }
};
}
