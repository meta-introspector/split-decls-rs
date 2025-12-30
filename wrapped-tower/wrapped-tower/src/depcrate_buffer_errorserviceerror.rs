// Generated macro for ServiceError (struct)
macro_rules! Depcrate_buffer_errorServiceError {
() => {
// Module: crate::buffer::error
// Provides: {"ServiceError"}
// Dependencies: {}
# [doc = " An error produced by a [`Service`] wrapped by a [`Buffer`]"] # [doc = ""] # [doc = " [`Service`]: crate::Service"] # [doc = " [`Buffer`]: crate::buffer::Buffer"] # [derive (Debug)] pub struct ServiceError { inner : Arc < BoxError > , }
};
}
