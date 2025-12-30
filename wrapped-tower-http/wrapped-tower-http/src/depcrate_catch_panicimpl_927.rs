// Generated macro for impl_927 (impl)
macro_rules! Depcrate_catch_panicimpl_927 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_927"}
// Dependencies: {}
impl < S , T , ReqBody , ResBody > Service < Request < ReqBody > > for CatchPanic < S , T > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , ResBody : Body < Data = Bytes > + Send + 'static , ResBody :: Error : Into < BoxError > , T : ResponseForPanic + Clone , T :: ResponseBody : Body < Data = Bytes > + Send + 'static , < T :: ResponseBody as Body > :: Error : Into < BoxError > , { type Response = Response < UnsyncBoxBody < Bytes , BoxError > > ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future , T > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { match std :: panic :: catch_unwind (AssertUnwindSafe (| | self . inner . call (req))) { Ok (future) => ResponseFuture { kind : Kind :: Future { future : AssertUnwindSafe (future) . catch_unwind () , panic_handler : Some (self . panic_handler . clone ()) , } , } , Err (panic_err) => ResponseFuture { kind : Kind :: Panicked { panic_err : Some (panic_err) , panic_handler : Some (self . panic_handler . clone ()) , } , } , } } }
};
}
