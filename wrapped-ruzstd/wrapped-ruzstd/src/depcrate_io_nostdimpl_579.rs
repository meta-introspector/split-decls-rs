// Generated macro for impl_579 (impl)
macro_rules! Depcrate_io_nostdimpl_579 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_579"}
// Dependencies: {}
impl ErrorKind { fn as_str (& self) -> & 'static str { use ErrorKind :: * ; match * self { Interrupted => "operation interrupted" , UnexpectedEof => "unexpected end of file" , WouldBlock => "operation would block" , Other => "other error" , WriteAllEof => "write_all hit EOF" , } } }
};
}
