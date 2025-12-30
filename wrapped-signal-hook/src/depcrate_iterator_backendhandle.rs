// Generated macro for Handle (struct)
macro_rules! Depcrate_iterator_backendHandle {
() => {
// Module: crate::iterator::backend
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " A struct to control an instance of an associated type"] # [doc = " (like for example [`Signals`][super::Signals])."] # [doc = ""] # [doc = " It allows to register more signal handlers and to shutdown the signal"] # [doc = " delivery. You can [`clone`][Handle::clone] this type which isn't a"] # [doc = " very expensive operation. The cloned instances can be shared between"] # [doc = " multiple threads."] # [derive (Debug , Clone)] pub struct Handle { pending : Arc < dyn AddSignal > , write : Arc < dyn SelfPipeWrite > , delivery_state : Arc < DeliveryState > , }
};
}
