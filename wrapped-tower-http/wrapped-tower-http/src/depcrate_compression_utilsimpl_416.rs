// Generated macro for impl_416 (impl)
macro_rules! Depcrate_compression_utilsimpl_416 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_416"}
// Dependencies: {}
impl < M : DecorateAsyncRead > WrapBody < M > { # [allow (dead_code)] pub (crate) fn new < B > (body : B , quality : CompressionLevel) -> Self where B : Body , M : DecorateAsyncRead < Input = AsyncReadBody < B > > , { let stream = BodyIntoStream :: new (body) ; let stream = StreamErrorIntoIoError :: < _ , B :: Error > :: new (stream) ; let read = StreamReader :: new (stream) ; let read = M :: apply (read , quality) ; Self { read , buf : BytesMut :: with_capacity (Self :: INTERNAL_BUF_CAPACITY) , read_all_data : false , } } }
};
}
