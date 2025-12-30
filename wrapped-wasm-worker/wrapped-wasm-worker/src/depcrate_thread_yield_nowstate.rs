// Generated macro for State (enum)
macro_rules! Depcrate_thread_yield_nowState {
() => {
// Module: crate::thread::yield_now
// Provides: {"State"}
// Dependencies: {}
# [doc = " State of [`YieldNowFuture`]."] # [derive (Debug)] enum State { # [doc = " Used [`Scheduler.postTask()`](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)."] Scheduler { # [doc = " [`Future`]."] future : JsFuture , # [doc = " Abort when dropped."] controller : AbortController , } , # [doc = " Used [`Window.requestIdleCallback()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)."] Idle { # [doc = " [`WakerData`]."] waker : Rc < RefCell < WakerData > > , # [doc = " Callback to wake up the [`Future`]."] _callback : Closure < dyn FnMut () > , # [doc = " Abort when dropped."] handle : u32 , } , # [doc = " Used [`MessagePort.postMessage()`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)."] Channel { # [doc = " [`WakerData`]."] waker : Rc < RefCell < WakerData > > , # [doc = " Callback to wake up the [`Future`]."] _callback : Closure < dyn FnMut () > , # [doc = " Abort when dropped."] port : MessagePort , } , # [doc = " Yielding to the event loop not supported."] None , }
};
}
