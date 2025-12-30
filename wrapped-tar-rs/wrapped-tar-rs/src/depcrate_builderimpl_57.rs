// Generated macro for impl_57 (impl)
macro_rules! Depcrate_builderimpl_57 {
() => {
// Module: crate::builder
// Provides: {"impl_57"}
// Dependencies: {}
impl EntryWriter < '_ > { fn start < 'a > (obj : & 'a mut dyn SeekWrite , header : & 'a mut Header , path : & Path ,) -> io :: Result < EntryWriter < 'a > > { prepare_header_path (obj . as_write () , header , path) ? ; obj . write_all ([0u8 ; BLOCK_SIZE as usize] . as_ref ()) ? ; Ok (EntryWriter { obj , header , written : 0 , }) } # [doc = " Finish writing the current entry in the archive."] pub fn finish (self) -> io :: Result < () > { let mut this = std :: mem :: ManuallyDrop :: new (self) ; this . do_finish () } fn do_finish (& mut self) -> io :: Result < () > { let buf = [0u8 ; BLOCK_SIZE as usize] ; let remaining = BLOCK_SIZE . wrapping_sub (self . written) % BLOCK_SIZE ; self . obj . write_all (& buf [.. remaining as usize]) ? ; let written = (self . written + remaining) as i64 ; self . obj . seek (io :: SeekFrom :: Current (- written - BLOCK_SIZE as i64)) ? ; self . header . set_size (self . written) ; self . header . set_cksum () ; self . obj . write_all (self . header . as_bytes ()) ? ; self . obj . seek (io :: SeekFrom :: Current (written)) ? ; Ok (()) } }
};
}
