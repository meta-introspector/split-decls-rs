// Generated macro for RequestBodyLimitLayer (struct)
macro_rules! Depcrate_limit_layerRequestBodyLimitLayer {
() => {
// Module: crate::limit::layer
// Provides: {"RequestBodyLimitLayer"}
// Dependencies: {}
# [doc = " Layer that applies the [`RequestBodyLimit`] middleware that intercepts requests"] # [doc = " with body lengths greater than the configured limit and converts them into"] # [doc = " `413 Payload Too Large` responses."] # [doc = ""] # [doc = " See the [module docs](crate::limit) for an example."] # [doc = ""] # [doc = " [`RequestBodyLimit`]: super::RequestBodyLimit"] # [derive (Clone , Copy , Debug)] pub struct RequestBodyLimitLayer { limit : usize , }
};
}
