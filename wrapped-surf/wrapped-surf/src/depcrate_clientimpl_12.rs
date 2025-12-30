// Generated macro for impl_12 (impl)
macro_rules! Depcrate_clientimpl_12 {
() => {
// Module: crate::client
// Provides: {"impl_12"}
// Dependencies: {}
impl Clone for Client { # [doc = " Clones the Client."] # [doc = ""] # [doc = " This copies the middleware stack from the original, but shares"] # [doc = " the `HttpClient` and http client config of the original."] # [doc = " Note that individual middleware in the middleware stack are"] # [doc = " still shared by reference."] fn clone (& self) -> Self { Self { config : self . config . clone () , http_client : self . http_client . clone () , middleware : Arc :: new (self . middleware . iter () . cloned () . collect ()) , } } }
};
}
