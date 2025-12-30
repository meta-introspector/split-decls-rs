// Generated macro for RegisterThreadFuture (struct)
macro_rules! Depcrate_thread_unsupported_audio_workletRegisterThreadFuture {
() => {
// Module: crate::thread::unsupported::audio_worklet
// Provides: {"RegisterThreadFuture"}
// Dependencies: {}
# [doc = " Implementation for [`crate::web::audio_worklet::RegisterThreadFuture`]."] # [derive (Debug)] pub (in super :: super) struct RegisterThreadFuture { # [doc = " Only possible state is an error."] error : Option < Error > , # [doc = " Make sure it doesn't implement [`Send`] or [`Sync`]."] _marker : PhantomData < * const () > , }
};
}
