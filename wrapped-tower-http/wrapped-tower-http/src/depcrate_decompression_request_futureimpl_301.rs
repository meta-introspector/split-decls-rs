// Generated macro for impl_301 (impl)
macro_rules! Depcrate_decompression_request_futureimpl_301 {
() => {
// Module: crate::decompression::request::future
// Provides: {"impl_301"}
// Dependencies: {}
impl < F , B , E > Future for RequestDecompressionFuture < F , B , E > where F : Future < Output = Result < Response < B > , E > > , B : Body + Send + 'static , B :: Data : Buf + 'static , B :: Error : Into < BoxError > + 'static , { type Output = Result < Response < UnsyncBoxBody < B :: Data , BoxError > > , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . kind . project () { StateProj :: Inner { fut } => fut . poll (cx) . map_ok (| res | { res . map (| body | UnsyncBoxBody :: new (body . map_err (Into :: into) . boxed_unsync ())) }) , StateProj :: Unsupported { accept } => { let res = Response :: builder () . header (header :: ACCEPT_ENCODING , accept . to_header_value () . unwrap_or (HeaderValue :: from_static ("identity")) ,) . status (StatusCode :: UNSUPPORTED_MEDIA_TYPE) . body (UnsyncBoxBody :: new (Empty :: new () . map_err (Into :: into) . boxed_unsync () ,)) . unwrap () ; Poll :: Ready (Ok (res)) } } } }
};
}
