// Generated macro for end (function)
macro_rules! Depcrate_deflateend {
() => {
// Module: crate::deflate
// Provides: {"end"}
// Dependencies: {}
# [doc = " # Returns"] # [doc = ""] # [doc = " - Err when deflate is not done. A common cause is insufficient output space"] # [doc = " - Ok otherwise"] pub fn end < 'a > (stream : & 'a mut DeflateStream) -> Result < & 'a mut z_stream , & 'a mut z_stream > { let status = stream . state . status ; let alloc = stream . alloc ; unsafe { stream . state . sym_buf . drop_in (& alloc) ; stream . state . bit_writer . pending . drop_in (& alloc) ; alloc . deallocate (stream . state . head . as_mut_ptr () , 1) ; if ! stream . state . prev . is_empty () { alloc . deallocate (stream . state . prev . as_mut_ptr () , stream . state . prev . len ()) ; } stream . state . window . drop_in (& alloc) ; } let stream = stream . as_z_stream_mut () ; let state = core :: mem :: replace (& mut stream . state , core :: ptr :: null_mut ()) ; unsafe { alloc . deallocate (state as * mut State , 1) ; } match status { Status :: Busy => Err (stream) , _ => Ok (stream) , } }
};
}
