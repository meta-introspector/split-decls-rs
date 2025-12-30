// Generated macro for impl_255 (impl)
macro_rules! Depcrate_add_extensionimpl_255 {
() => {
// Module: crate::add_extension
// Provides: {"impl_255"}
// Dependencies: {}
impl < ResBody , ReqBody , S , T > Service < Request < ReqBody > > for AddExtension < S , T > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , T : Clone + Send + Sync + 'static , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { req . extensions_mut () . insert (self . value . clone ()) ; self . inner . call (req) } }
};
}
