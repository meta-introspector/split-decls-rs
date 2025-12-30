// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl WriteColor for Buffer { # [inline] fn supports_color (& self) -> bool { match self . 0 { BufferInner :: NoColor (_) => false , BufferInner :: Ansi (_) => true , # [cfg (windows)] BufferInner :: Windows (_) => true , } } # [inline] fn supports_hyperlinks (& self) -> bool { match self . 0 { BufferInner :: NoColor (_) => false , BufferInner :: Ansi (_) => true , # [cfg (windows)] BufferInner :: Windows (_) => false , } } # [inline] fn set_color (& mut self , spec : & ColorSpec) -> io :: Result < () > { match self . 0 { BufferInner :: NoColor (ref mut w) => w . set_color (spec) , BufferInner :: Ansi (ref mut w) => w . set_color (spec) , # [cfg (windows)] BufferInner :: Windows (ref mut w) => w . set_color (spec) , } } # [inline] fn set_hyperlink (& mut self , link : & HyperlinkSpec) -> io :: Result < () > { match self . 0 { BufferInner :: NoColor (ref mut w) => w . set_hyperlink (link) , BufferInner :: Ansi (ref mut w) => w . set_hyperlink (link) , # [cfg (windows)] BufferInner :: Windows (ref mut w) => w . set_hyperlink (link) , } } # [inline] fn reset (& mut self) -> io :: Result < () > { match self . 0 { BufferInner :: NoColor (ref mut w) => w . reset () , BufferInner :: Ansi (ref mut w) => w . reset () , # [cfg (windows)] BufferInner :: Windows (ref mut w) => w . reset () , } } # [inline] fn is_synchronous (& self) -> bool { false } }
};
}
