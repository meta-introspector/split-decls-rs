// Generated macro for UnixListener (struct)
macro_rules! Depcrate_os_unix_net_listenerUnixListener {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"UnixListener"}
// Dependencies: {}
# [doc = " A structure representing a Unix domain socket server."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::thread;"] # [doc = " use std::os::unix::net::{UnixStream, UnixListener};"] # [doc = ""] # [doc = " fn handle_client(stream: UnixStream) {"] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let listener = UnixListener::bind(\"/path/to/the/socket\")?;"] # [doc = ""] # [doc = "     // accept connections and process them, spawning a new thread for each one"] # [doc = "     for stream in listener.incoming() {"] # [doc = "         match stream {"] # [doc = "             Ok(stream) => {"] # [doc = "                 /* connection succeeded */"] # [doc = "                 thread::spawn(|| handle_client(stream));"] # [doc = "             }"] # [doc = "             Err(err) => {"] # [doc = "                 /* connection failed */"] # [doc = "                 break;"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub struct UnixListener (Socket) ;
};
}
