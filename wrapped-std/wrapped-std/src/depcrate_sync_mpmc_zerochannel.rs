// Generated macro for Channel (struct)
macro_rules! Depcrate_sync_mpmc_zeroChannel {
() => {
// Module: crate::sync::mpmc::zero
// Provides: {"Channel"}
// Dependencies: {}
# [doc = " Zero-capacity channel."] pub (crate) struct Channel < T > { # [doc = " Inner representation of the channel."] inner : Mutex < Inner > , # [doc = " Indicates that dropping a `Channel<T>` may drop values of type `T`."] _marker : PhantomData < T > , }
};
}
