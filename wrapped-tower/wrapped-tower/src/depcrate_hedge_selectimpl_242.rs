// Generated macro for impl_242 (impl)
macro_rules! Depcrate_hedge_selectimpl_242 {
() => {
// Module: crate::hedge::select
// Provides: {"impl_242"}
// Dependencies: {}
impl < P , A , B , Request > Service < Request > for Select < P , A , B > where P : Policy < Request > , A : Service < Request > , A :: Error : Into < crate :: BoxError > , B : Service < Request , Response = A :: Response > , B :: Error : Into < crate :: BoxError > , { type Response = A :: Response ; type Error = crate :: BoxError ; type Future = ResponseFuture < A :: Future , B :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match (self . a . poll_ready (cx) , self . b . poll_ready (cx)) { (Poll :: Ready (Ok (())) , Poll :: Ready (Ok (()))) => Poll :: Ready (Ok (())) , (Poll :: Ready (Err (e)) , _) => Poll :: Ready (Err (e . into ())) , (_ , Poll :: Ready (Err (e))) => Poll :: Ready (Err (e . into ())) , _ => Poll :: Pending , } } fn call (& mut self , request : Request) -> Self :: Future { let b_fut = if let Some (cloned_req) = self . policy . clone_request (& request) { Some (self . b . call (cloned_req)) } else { None } ; ResponseFuture { a_fut : self . a . call (request) , b_fut , } } }
};
}
