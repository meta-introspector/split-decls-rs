// Generated macro for impl_13 (impl)
macro_rules! Depcrate_errorimpl_13 {
() => {
// Module: crate::error
// Provides: {"impl_13"}
// Dependencies: {}
impl Error { # [doc = " Create a new error with an associated source."] # [doc = ""] # [doc = " **NOTE:** The \"source\" should **NOT** be used to propagate cryptographic"] # [doc = " errors e.g. signature parsing or verification errors. The intended use"] # [doc = " cases are for propagating errors related to external signers, e.g."] # [doc = " communication/authentication errors with HSMs, KMS, etc."] # [cfg (feature = "alloc")] pub fn from_source (source : impl Into < Box < dyn core :: error :: Error + Send + Sync + 'static > > ,) -> Self { Self { source : Some (source . into ()) , } } }
};
}
