// Generated macro for impl_3245 (impl)
macro_rules! Depcrate_sync_mpscimpl_3245 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3245"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > Clone for Sender < T > { # [doc = " Clone a sender to send to other threads."] # [doc = ""] # [doc = " Note, be aware of the lifetime of the sender because all senders"] # [doc = " (including the original) need to be dropped in order for"] # [doc = " [`Receiver::recv`] to stop blocking."] fn clone (& self) -> Sender < T > { Sender { inner : self . inner . clone () } } }
};
}
