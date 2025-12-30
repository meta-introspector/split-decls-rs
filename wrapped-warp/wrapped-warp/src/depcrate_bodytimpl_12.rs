// Generated macro for impl_12 (impl)
macro_rules! Depcrate_bodytimpl_12 {
() => {
// Module: crate::bodyt
// Provides: {"impl_12"}
// Dependencies: {}
impl Body { pub (crate) fn empty () -> Self { Body (http_body_util :: Empty :: < Bytes > :: new () . map_err (crate :: Error :: new) . boxed () ,) } pub (crate) fn wrap < B > (body : B) -> Self where B : http_body :: Body + Send + Sync + 'static , B :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , { let body = body . map_frame (| f | f . map_data (| mut buf | buf . copy_to_bytes (buf . remaining ()))) . map_err (crate :: Error :: new) ; Body (http_body_util :: BodyExt :: boxed (body)) } pub (crate) fn wrap_stream < S , B , E > (stream : S) -> Self where S : futures_util :: Stream < Item = Result < B , E > > + Send + Sync + 'static , B : Into < Bytes > , E : Into < Box < dyn std :: error :: Error + Send + Sync > > + Send + 'static , { let body = http_body_util :: StreamBody :: new (stream . map (| item | { item . map (| buf | Frame :: data (buf . into ())) . map_err (crate :: Error :: new) })) ; Body (http_body_util :: BodyExt :: boxed (body)) } }
};
}
