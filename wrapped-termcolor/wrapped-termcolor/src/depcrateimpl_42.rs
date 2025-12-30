// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a , W : io :: Write > io :: Write for WriterInnerLock < 'a , W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match * self { WriterInnerLock :: Unreachable (_) => unreachable ! () , WriterInnerLock :: NoColor (ref mut wtr) => wtr . write (buf) , WriterInnerLock :: Ansi (ref mut wtr) => wtr . write (buf) , # [cfg (windows)] WriterInnerLock :: Windows { ref mut wtr , .. } => wtr . write (buf) , } } fn flush (& mut self) -> io :: Result < () > { match * self { WriterInnerLock :: Unreachable (_) => unreachable ! () , WriterInnerLock :: NoColor (ref mut wtr) => wtr . flush () , WriterInnerLock :: Ansi (ref mut wtr) => wtr . flush () , # [cfg (windows)] WriterInnerLock :: Windows { ref mut wtr , .. } => wtr . flush () , } } }
};
}
