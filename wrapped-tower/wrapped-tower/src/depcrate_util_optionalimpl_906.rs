// Generated macro for impl_906 (impl)
macro_rules! Depcrate_util_optionalimpl_906 {
() => {
// Module: crate::util::optional
// Provides: {"impl_906"}
// Dependencies: {}
impl < T , Request > Service < Request > for Optional < T > where T : Service < Request > , T :: Error : Into < crate :: BoxError > , { type Response = T :: Response ; type Error = crate :: BoxError ; type Future = ResponseFuture < T :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . inner { Some (ref mut inner) => match inner . poll_ready (cx) { Poll :: Ready (r) => Poll :: Ready (r . map_err (Into :: into)) , Poll :: Pending => Poll :: Pending , } , None => Poll :: Ready (Ok (())) , } } fn call (& mut self , request : Request) -> Self :: Future { let inner = self . inner . as_mut () . map (| i | i . call (request)) ; ResponseFuture :: new (inner) } }
};
}
