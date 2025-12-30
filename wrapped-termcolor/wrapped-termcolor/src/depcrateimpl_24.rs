// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a > io :: Write for IoStandardStreamLock < 'a > { # [inline (always)] fn write (& mut self , b : & [u8]) -> io :: Result < usize > { match * self { IoStandardStreamLock :: StdoutLock (ref mut s) => s . write (b) , IoStandardStreamLock :: StderrLock (ref mut s) => s . write (b) , } } # [inline (always)] fn flush (& mut self) -> io :: Result < () > { match * self { IoStandardStreamLock :: StdoutLock (ref mut s) => s . flush () , IoStandardStreamLock :: StderrLock (ref mut s) => s . flush () , } } }
};
}
