// Generated macro for Sender (struct)
macro_rules! Depcrate_sync_mpmcSender {
() => {
// Module: crate::sync::mpmc
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " The sending-half of Rust's synchronous [`channel`] type."] # [doc = ""] # [doc = " Messages can be sent through this channel with [`send`]."] # [doc = ""] # [doc = " Note: all senders (the original and its clones) need to be dropped for the receiver"] # [doc = " to stop blocking to receive messages with [`Receiver::recv`]."] # [doc = ""] # [doc = " [`send`]: Sender::send"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(mpmc_channel)]"] # [doc = ""] # [doc = " use std::sync::mpmc::channel;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let (sender, receiver) = channel();"] # [doc = " let sender2 = sender.clone();"] # [doc = ""] # [doc = " // First thread owns sender"] # [doc = " thread::spawn(move || {"] # [doc = "     sender.send(1).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " // Second thread owns sender2"] # [doc = " thread::spawn(move || {"] # [doc = "     sender2.send(2).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " let msg = receiver.recv().unwrap();"] # [doc = " let msg2 = receiver.recv().unwrap();"] # [doc = ""] # [doc = " assert_eq!(3, msg + msg2);"] # [doc = " ```"] # [unstable (feature = "mpmc_channel" , issue = "126840")] pub struct Sender < T > { flavor : SenderFlavor < T > , }
};
}
