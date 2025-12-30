// Generated macro for Writer (struct)
macro_rules! Depcrate_writerWriter {
() => {
// Module: crate::writer
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " A writer of various kinds of Unicode data."] # [doc = ""] # [doc = " A writer takes as input various forms of Unicode data and writes that data"] # [doc = " in a number of different output formats."] pub struct Writer { wtr : LineWriter < Box < dyn io :: Write + 'static > > , wrote_header : bool , opts : WriterOptions , }
};
}
