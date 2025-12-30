// Generated macro for impl_930 (impl)
macro_rules! Depcrate_catch_panicimpl_930 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_930"}
// Dependencies: {}
impl < F , ResBody , E , T > Future for ResponseFuture < F , T > where F : Future < Output = Result < Response < ResBody > , E > > , ResBody : Body < Data = Bytes > + Send + 'static , ResBody :: Error : Into < BoxError > , T : ResponseForPanic , T :: ResponseBody : Body < Data = Bytes > + Send + 'static , < T :: ResponseBody as Body > :: Error : Into < BoxError > , { type Output = Result < Response < UnsyncBoxBody < Bytes , BoxError > > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . kind . project () { KindProj :: Panicked { panic_err , panic_handler , } => { let panic_handler = panic_handler . take () . expect ("future polled after completion") ; let panic_err = panic_err . take () . expect ("future polled after completion") ; Poll :: Ready (Ok (response_for_panic (panic_handler , panic_err))) } KindProj :: Future { future , panic_handler , } => match ready ! (future . poll (cx)) { Ok (Ok (res)) => { Poll :: Ready (Ok (res . map (| body | { UnsyncBoxBody :: new (body . map_err (Into :: into) . boxed_unsync ()) }))) } Ok (Err (svc_err)) => Poll :: Ready (Err (svc_err)) , Err (panic_err) => Poll :: Ready (Ok (response_for_panic (panic_handler . take () . expect ("future polled after completion") , panic_err ,))) , } , } } }
};
}
