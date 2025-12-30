// Generated macro for impl_94 (impl)
macro_rules! Depcrate_buffer_layerimpl_94 {
() => {
// Module: crate::buffer::layer
// Provides: {"impl_94"}
// Dependencies: {}
impl < Request > BufferLayer < Request > { # [doc = " Creates a new [`BufferLayer`] with the provided `bound`."] # [doc = ""] # [doc = " `bound` gives the maximal number of requests that can be queued for the service before"] # [doc = " backpressure is applied to callers."] # [doc = ""] # [doc = " # A note on choosing a `bound`"] # [doc = ""] # [doc = " When [`Buffer`]'s implementation of [`poll_ready`] returns [`Poll::Ready`], it reserves a"] # [doc = " slot in the channel for the forthcoming [`call`]. However, if this call doesn't arrive,"] # [doc = " this reserved slot may be held up for a long time. As a result, it's advisable to set"] # [doc = " `bound` to be at least the maximum number of concurrent requests the [`Buffer`] will see."] # [doc = " If you do not, all the slots in the buffer may be held up by futures that have just called"] # [doc = " [`poll_ready`] but will not issue a [`call`], which prevents other senders from issuing new"] # [doc = " requests."] # [doc = ""] # [doc = " [`Poll::Ready`]: std::task::Poll::Ready"] # [doc = " [`call`]: crate::Service::call"] # [doc = " [`poll_ready`]: crate::Service::poll_ready"] pub const fn new (bound : usize) -> Self { BufferLayer { bound , _p : PhantomData , } } }
};
}
