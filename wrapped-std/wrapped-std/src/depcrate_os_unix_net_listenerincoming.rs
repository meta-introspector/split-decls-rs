// Generated macro for Incoming (struct)
macro_rules! Depcrate_os_unix_net_listenerIncoming {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"Incoming"}
// Dependencies: {}
# [doc = " An iterator over incoming connections to a [`UnixListener`]."] # [doc = ""] # [doc = " It will never return [`None`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::thread;"] # [doc = " use std::os::unix::net::{UnixStream, UnixListener};"] # [doc = ""] # [doc = " fn handle_client(stream: UnixStream) {"] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] # [doc = ""] # [doc = "     for stream in listener.incoming() {"] # [doc = "         match stream {"] # [doc = "             Ok(stream) => {"] # [doc = "                 thread::spawn(|| handle_client(stream));"] # [doc = "             }"] # [doc = "             Err(err) => {"] # [doc = "                 break;"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub struct Incoming < 'a > { listener : & 'a UnixListener , }
};
}
