// Generated macro for ReadyOneshot (struct)
macro_rules! Depcrate_util_readyReadyOneshot {
() => {
// Module: crate::util::ready
// Provides: {"ReadyOneshot"}
// Dependencies: {}
# [doc = " A [`Future`] that yields the service when it is ready to accept a request."] # [doc = ""] # [doc = " [`ReadyOneshot`] values are produced by [`ServiceExt::ready_oneshot`]."] # [doc = ""] # [doc = " [`ServiceExt::ready_oneshot`]: crate::util::ServiceExt::ready_oneshot"] pub struct ReadyOneshot < T , Request > { inner : Option < T > , _p : PhantomData < fn () -> Request > , }
};
}
