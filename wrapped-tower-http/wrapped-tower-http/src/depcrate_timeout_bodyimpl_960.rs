// Generated macro for impl_960 (impl)
macro_rules! Depcrate_timeout_bodyimpl_960 {
() => {
// Module: crate::timeout::body
// Provides: {"impl_960"}
// Dependencies: {}
impl < B > Body for TimeoutBody < B > where B : Body , B :: Error : Into < BoxError > , { type Data = B :: Data ; type Error = Box < dyn std :: error :: Error + Send + Sync > ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < http_body :: Frame < Self :: Data > , Self :: Error > > > { let mut this = self . project () ; let sleep_pinned = if let Some (some) = this . sleep . as_mut () . as_pin_mut () { some } else { this . sleep . set (Some (sleep (* this . timeout))) ; this . sleep . as_mut () . as_pin_mut () . unwrap () } ; if let Poll :: Ready (()) = sleep_pinned . poll (cx) { return Poll :: Ready (Some (Err (Box :: new (TimeoutError (()))))) ; } let frame = ready ! (this . body . poll_frame (cx)) ; this . sleep . set (None) ; Poll :: Ready (frame . transpose () . map_err (Into :: into) . transpose ()) } }
};
}
