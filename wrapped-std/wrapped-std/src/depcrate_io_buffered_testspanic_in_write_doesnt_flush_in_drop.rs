// Generated macro for panic_in_write_doesnt_flush_in_drop (function)
macro_rules! Depcrate_io_buffered_testspanic_in_write_doesnt_flush_in_drop {
() => {
// Module: crate::io::buffered::tests
// Provides: {"panic_in_write_doesnt_flush_in_drop"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_os = "wasi") , ignore)] fn panic_in_write_doesnt_flush_in_drop () { static WRITES : AtomicUsize = AtomicUsize :: new (0) ; struct PanicWriter ; impl Write for PanicWriter { fn write (& mut self , _ : & [u8]) -> io :: Result < usize > { WRITES . fetch_add (1 , Ordering :: SeqCst) ; panic ! () ; } fn flush (& mut self) -> io :: Result < () > { Ok (()) } } thread :: spawn (| | { let mut writer = BufWriter :: new (PanicWriter) ; let _ = writer . write (b"hello world") ; let _ = writer . flush () ; }) . join () . unwrap_err () ; assert_eq ! (WRITES . load (Ordering :: SeqCst) , 1) ; }
};
}
