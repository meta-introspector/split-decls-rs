// Generated macro for TryIter (struct)
macro_rules! Depcrate_sync_mpmcTryIter {
() => {
// Module: crate::sync::mpmc
// Provides: {"TryIter"}
// Dependencies: {}
# [doc = " An iterator that attempts to yield all pending values for a [`Receiver`],"] # [doc = " created by [`try_iter`]."] # [doc = ""] # [doc = " [`None`] will be returned when there are no pending values remaining or"] # [doc = " if the corresponding channel has hung up."] # [doc = ""] # [doc = " This iterator will never block the caller in order to wait for data to"] # [doc = " become available. Instead, it will return [`None`]."] # [doc = ""] # [doc = " [`try_iter`]: Receiver::try_iter"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(mpmc_channel)]"] # [doc = ""] # [doc = " use std::sync::mpmc::channel;"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let (sender, receiver) = channel();"] # [doc = ""] # [doc = " // Nothing is in the buffer yet"] # [doc = " assert!(receiver.try_iter().next().is_none());"] # [doc = " println!(\"Nothing in the buffer...\");"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     sender.send(1).unwrap();"] # [doc = "     sender.send(2).unwrap();"] # [doc = "     sender.send(3).unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " println!(\"Going to sleep...\");"] # [doc = " thread::sleep(Duration::from_secs(2)); // block for two seconds"] # [doc = ""] # [doc = " for x in receiver.try_iter() {"] # [doc = "     println!(\"Got: {x}\");"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "mpmc_channel" , issue = "126840")] # [derive (Debug)] pub struct TryIter < 'a , T : 'a > { rx : & 'a Receiver < T > , }
};
}
