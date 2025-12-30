// Generated macro for impl_17 (impl)
macro_rules! Depcrate_test_helpersimpl_17 {
() => {
// Module: crate::test_helpers
// Provides: {"impl_17"}
// Dependencies: {}
impl Body { pub (crate) fn new < B > (body : B) -> Self where B : http_body :: Body < Data = Bytes > + Send + 'static , B :: Error : Into < BoxError > , { Self (body . map_err (Into :: into) . boxed_unsync ()) } pub (crate) fn empty () -> Self { Self :: new (http_body_util :: Empty :: new ()) } pub (crate) fn from_stream < S > (stream : S) -> Self where S : TryStream + Send + 'static , S :: Ok : Into < Bytes > , S :: Error : Into < BoxError > , { Self :: new (StreamBody { stream : SyncWrapper :: new (stream) , }) } pub (crate) fn with_trailers (self , trailers : HeaderMap) -> WithTrailers < Self > { WithTrailers { inner : self , trailers : Some (trailers) , } } }
};
}
