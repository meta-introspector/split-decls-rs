// Generated macro for impl_969 (impl)
macro_rules! Depcrate_io_buffered_testsimpl_969 {
() => {
// Module: crate::io::buffered::tests
// Provides: {"impl_969"}
// Dependencies: {}
impl Write for ProgrammableSink { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { if self . always_write_error { return Err (io :: Error :: new (io :: ErrorKind :: Other , "test - always_write_error")) ; } match self . max_writes { Some (0) if self . error_after_max_writes => { return Err (io :: Error :: new (io :: ErrorKind :: Other , "test - max_writes")) ; } Some (0) => return Ok (0) , Some (ref mut count) => * count -= 1 , None => { } } let len = match self . accept_prefix { None => data . len () , Some (prefix) => data . len () . min (prefix) , } ; let data = & data [.. len] ; self . buffer . extend_from_slice (data) ; Ok (len) } fn flush (& mut self) -> io :: Result < () > { if self . always_flush_error { Err (io :: Error :: new (io :: ErrorKind :: Other , "test - always_flush_error")) } else { Ok (()) } } }
};
}
