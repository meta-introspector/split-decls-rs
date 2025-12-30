// Generated macro for impl_873 (impl)
macro_rules! Depcrate_corsimpl_873 {
() => {
// Module: crate::cors
// Provides: {"impl_873"}
// Dependencies: {}
impl < S , ReqBody , ResBody > Service < Request < ReqBody > > for Cors < S > where S : Service < Request < ReqBody > , Response = Response < ResBody > > , ResBody : Default , { type Response = S :: Response ; type Error = S :: Error ; type Future = ResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ensure_usable_cors_rules (& self . layer) ; self . inner . poll_ready (cx) } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let (parts , body) = req . into_parts () ; let origin = parts . headers . get (& header :: ORIGIN) ; let mut headers = HeaderMap :: new () ; headers . extend (self . layer . allow_credentials . to_header (origin , & parts)) ; headers . extend (self . layer . allow_private_network . to_header (origin , & parts)) ; headers . extend (self . layer . vary . to_header ()) ; let allow_origin_future = self . layer . allow_origin . to_future (origin , & parts) ; if parts . method == Method :: OPTIONS { headers . extend (self . layer . allow_methods . to_header (& parts)) ; headers . extend (self . layer . allow_headers . to_header (& parts)) ; headers . extend (self . layer . max_age . to_header (origin , & parts)) ; ResponseFuture { inner : Kind :: PreflightCall { allow_origin_future , headers , } , } } else { headers . extend (self . layer . expose_headers . to_header (& parts)) ; let req = Request :: from_parts (parts , body) ; ResponseFuture { inner : Kind :: CorsCall { allow_origin_future , allow_origin_complete : false , future : self . inner . call (req) , headers , } , } } } }
};
}
