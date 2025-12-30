// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < E > GraphQLEndpoint < E > { # [doc = " Set the multipart options of the endpoint."] # [must_use] pub fn multipart_opts (self , opts : MultipartOptions) -> Self { Self { opts , .. self } } # [doc = " Set whether batch requests are supported in the endpoint."] # [must_use] pub fn batch (self , batch : bool) -> Self { Self { batch , .. self } } }
};
}
