// Generated macro for Receiver (struct)
macro_rules! Depcrate_sync_mpmcReceiver {
() => {
// Module: crate::sync::mpmc
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " The receiving half of Rust's [`channel`] (or [`sync_channel`]) type."] # [doc = " Different threads can share this [`Receiver`] by cloning it."] # [doc = ""] # [doc = " Messages sent to the channel can be retrieved using [`recv`]."] # [doc = ""] # [doc = " [`recv`]: Receiver::recv"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(mpmc_channel)]"] # [doc = ""] # [doc = " use std::sync::mpmc::channel;"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let (send, recv) = channel();"] # [doc = ""] # [doc = " let tx_thread = thread::spawn(move || {"] # [doc = "     send.send(\"Hello world!\").unwrap();"] # [doc = "     thread::sleep(Duration::from_secs(2)); // block for two seconds"] # [doc = "     send.send(\"Delayed for 2 seconds\").unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " let (rx1, rx2) = (recv.clone(), recv.clone());"] # [doc = " let rx_thread_1 = thread::spawn(move || {"] # [doc = "     println!(\"{}\", rx1.recv().unwrap()); // Received immediately"] # [doc = " });"] # [doc = " let rx_thread_2 = thread::spawn(move || {"] # [doc = "     println!(\"{}\", rx2.recv().unwrap()); // Received after 2 seconds"] # [doc = " });"] # [doc = ""] # [doc = " tx_thread.join().unwrap();"] # [doc = " rx_thread_1.join().unwrap();"] # [doc = " rx_thread_2.join().unwrap();"] # [doc = " ```"] # [unstable (feature = "mpmc_channel" , issue = "126840")] pub struct Receiver < T > { flavor : ReceiverFlavor < T > , }
};
}
