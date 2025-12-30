// Generated macro for Iter (struct)
macro_rules! Depcrate_sync_mpscIter {
() => {
// Module: crate::sync::mpsc
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over messages on a [`Receiver`], created by [`iter`]."] # [doc = ""] # [doc = " This iterator will block whenever [`next`] is called,"] # [doc = " waiting for a new message, and [`None`] will be returned"] # [doc = " when the corresponding channel has hung up."] # [doc = ""] # [doc = " [`iter`]: Receiver::iter"] # [doc = " [`next`]: Iterator::next"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::sync::mpsc::channel;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let (send, recv) = channel();"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     send.send(1u8).unwrap();"] # [doc = "     send.send(2u8).unwrap();"] # [doc = "     send.send(3u8).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " for x in recv.iter() {"] # [doc = "     println!(\"Got: {x}\");"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct Iter < 'a , T : 'a > { rx : & 'a Receiver < T > , }
};
}
