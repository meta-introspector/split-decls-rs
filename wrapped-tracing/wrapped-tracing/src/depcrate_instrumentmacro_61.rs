// Generated macro for macro_61 (macro)
macro_rules! Depcrate_instrumentmacro_61 {
() => {
// Module: crate::instrument
// Provides: {"macro_61"}
// Dependencies: {}
pin_project ! { # [doc = " A [`Future`] that has been instrumented with a `tracing` [`Subscriber`]."] # [doc = ""] # [doc = " This type is returned by the [`WithSubscriber`] extension trait. See that"] # [doc = " trait's documentation for details."] # [doc = ""] # [doc = " [`Future`]: std::future::Future"] # [doc = " [`Subscriber`]: crate::Subscriber"] # [derive (Clone , Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub struct WithDispatch < T > { # [pin] inner : T , dispatcher : Dispatch , } }
};
}
