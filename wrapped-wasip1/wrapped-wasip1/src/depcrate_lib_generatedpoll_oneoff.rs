// Generated macro for poll_oneoff (function)
macro_rules! Depcrate_lib_generatedpoll_oneoff {
() => {
// Module: crate::lib_generated
// Provides: {"poll_oneoff"}
// Dependencies: {}
# [doc = " Concurrently poll for the occurrence of a set of events."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `in_` - The events to which to subscribe."] # [doc = " * `out` - The events that have occurred."] # [doc = " * `nsubscriptions` - Both the number of subscriptions and events."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The number of events stored."] pub unsafe fn poll_oneoff (in_ : * const Subscription , out : * mut Event , nsubscriptions : Size ,) -> Result < Size , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: poll_oneoff (in_ as i32 , out as i32 , nsubscriptions as i32 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size)) , _ => Err (Errno (ret as u16)) , } }
};
}
