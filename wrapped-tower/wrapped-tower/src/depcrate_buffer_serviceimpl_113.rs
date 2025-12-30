// Generated macro for impl_113 (impl)
macro_rules! Depcrate_buffer_serviceimpl_113 {
() => {
// Module: crate::buffer::service
// Provides: {"impl_113"}
// Dependencies: {}
impl < Req , Rsp , F , E > Service < Req > for Buffer < Req , F > where F : Future < Output = Result < Rsp , E > > + Send + 'static , E : Into < crate :: BoxError > , Req : Send + 'static , { type Response = Rsp ; type Error = crate :: BoxError ; type Future = ResponseFuture < F > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if self . tx . is_closed () { return Poll :: Ready (Err (self . get_worker_error ())) ; } self . tx . poll_reserve (cx) . map_err (| _ | self . get_worker_error ()) } fn call (& mut self , request : Req) -> Self :: Future { tracing :: trace ! ("sending request to buffer worker") ; let span = tracing :: Span :: current () ; let (tx , rx) = oneshot :: channel () ; match self . tx . send_item (Message { request , span , tx }) { Ok (_) => ResponseFuture :: new (rx) , Err (_) => { tracing :: trace ! ("buffer channel closed") ; ResponseFuture :: failed (self . get_worker_error ()) } } } }
};
}
