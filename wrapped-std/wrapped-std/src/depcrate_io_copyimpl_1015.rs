// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_io_copyimpl_1015 {
() => {
// Module: crate::io::copy
// Provides: {"impl_1015"}
// Dependencies: {}
impl < I > BufferedReaderSpec for BufReader < I > where Self : Read , I : ? Sized , { fn buffer_size (& self) -> usize { self . capacity () } fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > { let mut len = 0 ; loop { match self . read (& mut []) { Ok (_) => { } Err (e) if e . is_interrupted () => continue , Err (e) => return Err (e) , } let buf = self . buffer () ; if self . buffer () . len () == 0 { return Ok (len) ; } to . write_all (buf) ? ; len += buf . len () as u64 ; self . discard_buffer () ; } } }
};
}
