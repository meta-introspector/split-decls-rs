// Generated macro for StreamOwned (struct)
macro_rules! Depcrate_streamStreamOwned {
() => {
// Module: crate::stream
// Provides: {"StreamOwned"}
// Dependencies: {}
# [doc = " This type implements `io::Read` and `io::Write`, encapsulating"] # [doc = " and owning a Connection `C` and an underlying transport `T`, such as a socket."] # [doc = ""] # [doc = " Relies on [`ConnectionCommon::complete_io()`] to perform the necessary I/O."] # [doc = ""] # [doc = " This allows you to use a rustls Connection like a normal stream."] # [expect (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct StreamOwned < C : Sized , T : Read + Write + Sized > { # [doc = " Our connection"] pub conn : C , # [doc = " The underlying transport, like a socket"] pub sock : T , }
};
}
