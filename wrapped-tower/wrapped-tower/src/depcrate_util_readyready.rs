// Generated macro for Ready (struct)
macro_rules! Depcrate_util_readyReady {
() => {
// Module: crate::util::ready
// Provides: {"Ready"}
// Dependencies: {}
# [doc = " A future that yields a mutable reference to the service when it is ready to accept a request."] # [doc = ""] # [doc = " [`Ready`] values are produced by [`ServiceExt::ready`]."] # [doc = ""] # [doc = " [`ServiceExt::ready`]: crate::util::ServiceExt::ready"] pub struct Ready < 'a , T , Request > (ReadyOneshot < & 'a mut T , Request >) ;
};
}
