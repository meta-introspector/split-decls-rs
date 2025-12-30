// Generated macro for IntoIter (struct)
macro_rules! Depcrate_sync_mpscIntoIter {
() => {
// Module: crate::sync::mpsc
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over messages on a [`Receiver`],"] # [doc = " created by [`into_iter`]."] # [doc = ""] # [doc = " This iterator will block whenever [`next`]"] # [doc = " is called, waiting for a new message, and [`None`] will be"] # [doc = " returned if the corresponding channel has hung up."] # [doc = ""] # [doc = " [`into_iter`]: Receiver::into_iter"] # [doc = " [`next`]: Iterator::next"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::sync::mpsc::channel;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let (send, recv) = channel();"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     send.send(1u8).unwrap();"] # [doc = "     send.send(2u8).unwrap();"] # [doc = "     send.send(3u8).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " for x in recv.into_iter() {"] # [doc = "     println!(\"Got: {x}\");"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "receiver_into_iter" , since = "1.1.0")] # [derive (Debug)] pub struct IntoIter < T > { rx : Receiver < T > , }
};
}
