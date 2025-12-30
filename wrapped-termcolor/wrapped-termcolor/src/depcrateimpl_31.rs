// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a > StandardStreamLock < 'a > { # [cfg (not (windows))] fn from_stream (stream : & StandardStream) -> StandardStreamLock < '_ > { let locked = match * stream . wtr . get_ref () { WriterInner :: NoColor (ref w) => { WriterInnerLock :: NoColor (NoColor (w . 0 . lock ())) } WriterInner :: Ansi (ref w) => { WriterInnerLock :: Ansi (Ansi (w . 0 . lock ())) } } ; StandardStreamLock { wtr : stream . wtr . wrap (locked) } } # [cfg (windows)] fn from_stream (stream : & StandardStream) -> StandardStreamLock { let locked = match * stream . wtr . get_ref () { WriterInner :: NoColor (ref w) => { WriterInnerLock :: NoColor (NoColor (w . 0 . lock ())) } WriterInner :: Ansi (ref w) => { WriterInnerLock :: Ansi (Ansi (w . 0 . lock ())) } # [cfg (windows)] WriterInner :: Windows { ref wtr , ref console } => { WriterInnerLock :: Windows { wtr : wtr . lock () , console : console . lock () . unwrap () , } } } ; StandardStreamLock { wtr : stream . wtr . wrap (locked) } } }
};
}
