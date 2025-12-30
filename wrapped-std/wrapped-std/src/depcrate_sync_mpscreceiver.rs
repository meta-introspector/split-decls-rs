// Generated macro for Receiver (struct)
macro_rules! Depcrate_sync_mpscReceiver {
() => {
// Module: crate::sync::mpsc
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " The receiving half of Rust's [`channel`] (or [`sync_channel`]) type."] # [doc = " This half can only be owned by one thread."] # [doc = ""] # [doc = " Messages sent to the channel can be retrieved using [`recv`]."] # [doc = ""] # [doc = " [`recv`]: Receiver::recv"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::sync::mpsc::channel;"] # [doc = " use std::thread;"] # [doc = " use std::time::Duration;"] # [doc = ""] # [doc = " let (send, recv) = channel();"] # [doc = ""] # [doc = " thread::spawn(move || {"] # [doc = "     send.send(\"Hello world!\").unwrap();"] # [doc = "     thread::sleep(Duration::from_secs(2)); // block for two seconds"] # [doc = "     send.send(\"Delayed for 2 seconds\").unwrap();"] # [doc = " });"] # [doc = ""] # [doc = " println!(\"{}\", recv.recv().unwrap()); // Received immediately"] # [doc = " println!(\"Waiting...\");"] # [doc = " println!(\"{}\", recv.recv().unwrap()); // Received after 2 seconds"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "Receiver")] pub struct Receiver < T > { inner : mpmc :: Receiver < T > , }
};
}
