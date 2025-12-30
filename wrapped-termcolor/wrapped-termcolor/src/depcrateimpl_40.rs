// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < W : io :: Write > io :: Write for WriterInner < W > { # [inline (always)] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match * self { WriterInner :: NoColor (ref mut wtr) => wtr . write (buf) , WriterInner :: Ansi (ref mut wtr) => wtr . write (buf) , # [cfg (windows)] WriterInner :: Windows { ref mut wtr , .. } => wtr . write (buf) , } } # [inline (always)] fn flush (& mut self) -> io :: Result < () > { match * self { WriterInner :: NoColor (ref mut wtr) => wtr . flush () , WriterInner :: Ansi (ref mut wtr) => wtr . flush () , # [cfg (windows)] WriterInner :: Windows { ref mut wtr , .. } => wtr . flush () , } } }
};
}
