// Generated macro for dont_panic_in_drop_on_panicked_flush (function)
macro_rules! Depcrate_io_buffered_testsdont_panic_in_drop_on_panicked_flush {
() => {
// Module: crate::io::buffered::tests
// Provides: {"dont_panic_in_drop_on_panicked_flush"}
// Dependencies: {}
# [test] # [should_panic] fn dont_panic_in_drop_on_panicked_flush () { struct FailFlushWriter ; impl Write for FailFlushWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Err (io :: Error :: last_os_error ()) } } let writer = FailFlushWriter ; let _writer = BufWriter :: new (writer) ; panic ! () ; }
};
}
