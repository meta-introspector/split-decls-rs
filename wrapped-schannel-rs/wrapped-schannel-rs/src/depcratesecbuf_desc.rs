// Generated macro for secbuf_desc (function)
macro_rules! Depcratesecbuf_desc {
() => {
// Module: crate
// Provides: {"secbuf_desc"}
// Dependencies: {}
unsafe fn secbuf_desc (bufs : & mut [Identity :: SecBuffer]) -> Identity :: SecBufferDesc { Identity :: SecBufferDesc { ulVersion : Identity :: SECBUFFER_VERSION , cBuffers : bufs . len () as u32 , pBuffers : bufs . as_mut_ptr () , } }
};
}
