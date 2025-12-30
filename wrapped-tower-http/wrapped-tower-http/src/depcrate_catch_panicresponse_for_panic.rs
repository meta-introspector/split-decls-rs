// Generated macro for response_for_panic (function)
macro_rules! Depcrate_catch_panicresponse_for_panic {
() => {
// Module: crate::catch_panic
// Provides: {"response_for_panic"}
// Dependencies: {}
fn response_for_panic < T > (mut panic_handler : T , err : Box < dyn Any + Send + 'static > ,) -> Response < UnsyncBoxBody < Bytes , BoxError > > where T : ResponseForPanic , T :: ResponseBody : Body < Data = Bytes > + Send + 'static , < T :: ResponseBody as Body > :: Error : Into < BoxError > , { panic_handler . response_for_panic (err) . map (| body | UnsyncBoxBody :: new (body . map_err (Into :: into) . boxed_unsync ())) }
};
}
