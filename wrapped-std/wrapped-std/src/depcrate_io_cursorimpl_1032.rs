// Generated macro for impl_1032 (impl)
macro_rules! Depcrate_io_cursorimpl_1032 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1032"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > io :: Seek for Cursor < T > where T : AsRef < [u8] > , { fn seek (& mut self , style : SeekFrom) -> io :: Result < u64 > { let (base_pos , offset) = match style { SeekFrom :: Start (n) => { self . pos = n ; return Ok (n) ; } SeekFrom :: End (n) => (self . inner . as_ref () . len () as u64 , n) , SeekFrom :: Current (n) => (self . pos , n) , } ; match base_pos . checked_add_signed (offset) { Some (n) => { self . pos = n ; Ok (self . pos) } None => Err (io :: const_error ! (ErrorKind :: InvalidInput , "invalid seek to a negative or overflowing position" ,)) , } } fn stream_len (& mut self) -> io :: Result < u64 > { Ok (self . inner . as_ref () . len () as u64) } fn stream_position (& mut self) -> io :: Result < u64 > { Ok (self . pos) } }
};
}
