// Generated macro for impl_1009 (impl)
macro_rules! Depcrate_normalize_pathimpl_1009 {
() => {
// Module: crate::normalize_path
// Provides: {"impl_1009"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for NormalizePath < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , mut req : Request < ReqBody >) -> Self :: Future { match self . mode { NormalizeMode :: Trim => trim_trailing_slash (req . uri_mut ()) , NormalizeMode :: Append => append_trailing_slash (req . uri_mut ()) , } self . inner . call (req) } }
};
}
