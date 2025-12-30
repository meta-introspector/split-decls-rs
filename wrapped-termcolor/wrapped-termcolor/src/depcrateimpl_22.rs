// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl io :: Write for IoStandardStream { # [inline (always)] fn write (& mut self , b : & [u8]) -> io :: Result < usize > { match * self { IoStandardStream :: Stdout (ref mut s) => s . write (b) , IoStandardStream :: Stderr (ref mut s) => s . write (b) , IoStandardStream :: StdoutBuffered (ref mut s) => s . write (b) , IoStandardStream :: StderrBuffered (ref mut s) => s . write (b) , } } # [inline (always)] fn flush (& mut self) -> io :: Result < () > { match * self { IoStandardStream :: Stdout (ref mut s) => s . flush () , IoStandardStream :: Stderr (ref mut s) => s . flush () , IoStandardStream :: StdoutBuffered (ref mut s) => s . flush () , IoStandardStream :: StderrBuffered (ref mut s) => s . flush () , } } }
};
}
