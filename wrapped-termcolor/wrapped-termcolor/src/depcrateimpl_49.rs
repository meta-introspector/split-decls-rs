// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl io :: Write for Buffer { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match self . 0 { BufferInner :: NoColor (ref mut w) => w . write (buf) , BufferInner :: Ansi (ref mut w) => w . write (buf) , # [cfg (windows)] BufferInner :: Windows (ref mut w) => w . write (buf) , } } # [inline] fn flush (& mut self) -> io :: Result < () > { match self . 0 { BufferInner :: NoColor (ref mut w) => w . flush () , BufferInner :: Ansi (ref mut w) => w . flush () , # [cfg (windows)] BufferInner :: Windows (ref mut w) => w . flush () , } } }
};
}
