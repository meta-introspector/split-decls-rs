// Generated macro for ChunkVecBuffer (struct)
macro_rules! Depcrate_vecbufChunkVecBuffer {
() => {
// Module: crate::vecbuf
// Provides: {"ChunkVecBuffer"}
// Dependencies: {}
# [doc = " This is a byte buffer that is built from a deque of byte vectors."] # [doc = ""] # [doc = " This avoids extra copies when appending a new byte vector,"] # [doc = " at the expense of more complexity when reading out."] pub (crate) struct ChunkVecBuffer { # [doc = " How many bytes have been consumed in the first chunk."] # [doc = ""] # [doc = " Invariant: zero if `chunks.is_empty()`"] # [doc = " Invariant: 0 <= `prefix_used` < `chunks[0].len()`"] prefix_used : usize , chunks : VecDeque < Vec < u8 > > , # [doc = " The total upper limit (in bytes) of this object."] limit : Option < usize > , }
};
}
