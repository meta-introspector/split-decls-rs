// Generated macro for impl_897 (impl)
macro_rules! Depcrate_io_buffered_bufreaderimpl_897 {
() => {
// Module: crate::io::buffered::bufreader
// Provides: {"impl_897"}
// Dependencies: {}
impl < R : ? Sized + Seek > BufReader < R > { # [doc = " Seeks relative to the current position. If the new position lies within the buffer,"] # [doc = " the buffer will not be flushed, allowing for more efficient seeks."] # [doc = " This method does not return the location of the underlying reader, so the caller"] # [doc = " must track this information themselves if it is required."] # [stable (feature = "bufreader_seek_relative" , since = "1.53.0")] pub fn seek_relative (& mut self , offset : i64) -> io :: Result < () > { let pos = self . buf . pos () as u64 ; if offset < 0 { if let Some (_) = pos . checked_sub ((- offset) as u64) { self . buf . unconsume ((- offset) as usize) ; return Ok (()) ; } } else if let Some (new_pos) = pos . checked_add (offset as u64) { if new_pos <= self . buf . filled () as u64 { self . buf . consume (offset as usize) ; return Ok (()) ; } } self . seek (SeekFrom :: Current (offset)) . map (drop) } }
};
}
