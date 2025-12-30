// Generated macro for WriterInner (enum)
macro_rules! DepcrateWriterInner {
() => {
// Module: crate
// Provides: {"WriterInner"}
// Dependencies: {}
# [doc = " WriterInner is a (limited) generic representation of a writer. It is"] # [doc = " limited because W should only ever be stdout/stderr on Windows."] # [derive (Debug)] enum WriterInner < W > { NoColor (NoColor < W >) , Ansi (Ansi < W >) , # [cfg (windows)] Windows { wtr : W , console : Mutex < wincon :: Console > , } , }
};
}
