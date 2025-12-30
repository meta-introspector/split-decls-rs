// Generated macro for IntoInnerError (struct)
macro_rules! Depcrate_io_bufferedIntoInnerError {
() => {
// Module: crate::io::buffered
// Provides: {"IntoInnerError"}
// Dependencies: {}
# [doc = " An error returned by [`BufWriter::into_inner`] which combines an error that"] # [doc = " happened while writing out the buffer, and the buffered writer object"] # [doc = " which may be used to recover from the condition."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufWriter;"] # [doc = " use std::net::TcpStream;"] # [doc = ""] # [doc = " let mut stream = BufWriter::new(TcpStream::connect(\"127.0.0.1:34254\").unwrap());"] # [doc = ""] # [doc = " // do stuff with the stream"] # [doc = ""] # [doc = " // we want to get our `TcpStream` back, so let's try:"] # [doc = ""] # [doc = " let stream = match stream.into_inner() {"] # [doc = "     Ok(s) => s,"] # [doc = "     Err(e) => {"] # [doc = "         // Here, e is an IntoInnerError"] # [doc = "         panic!(\"An error occurred\");"] # [doc = "     }"] # [doc = " };"] # [doc = " ```"] # [derive (Debug)] # [stable (feature = "rust1" , since = "1.0.0")] pub struct IntoInnerError < W > (W , Error) ;
};
}
