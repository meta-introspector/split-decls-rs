// Generated macro for secbuf (function)
macro_rules! Depcratesecbuf {
() => {
// Module: crate
// Provides: {"secbuf"}
// Dependencies: {}
unsafe fn secbuf (buftype : u32 , bytes : Option < & mut [u8] >) -> Identity :: SecBuffer { let (ptr , len) = match bytes { Some (bytes) => (bytes . as_mut_ptr () , bytes . len () as u32) , None => (ptr :: null_mut () , 0) , } ; Identity :: SecBuffer { BufferType : buftype , cbBuffer : len , pvBuffer : ptr as * mut c_void , } }
};
}
