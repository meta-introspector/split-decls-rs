// Generated macro for Stream (struct)
macro_rules! Depcrate_streamStream {
() => {
// Module: crate::stream
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " This type implements `io::Read` and `io::Write`, encapsulating"] # [doc = " a Connection `C` and an underlying transport `T`, such as a socket."] # [doc = ""] # [doc = " Relies on [`ConnectionCommon::complete_io()`] to perform the necessary I/O."] # [doc = ""] # [doc = " This allows you to use a rustls Connection like a normal stream."] # [expect (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct Stream < 'a , C : 'a + ? Sized , T : 'a + Read + Write + ? Sized > { # [doc = " Our TLS connection"] pub conn : & 'a mut C , # [doc = " The underlying transport, like a socket"] pub sock : & 'a mut T , }
};
}
