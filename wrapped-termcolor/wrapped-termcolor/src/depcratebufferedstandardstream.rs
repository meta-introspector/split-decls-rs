// Generated macro for BufferedStandardStream (struct)
macro_rules! DepcrateBufferedStandardStream {
() => {
// Module: crate
// Provides: {"BufferedStandardStream"}
// Dependencies: {}
# [doc = " Like `StandardStream`, but does buffered writing."] # [derive (Debug)] pub struct BufferedStandardStream { wtr : LossyStandardStream < WriterInner < IoStandardStream > > , }
};
}
