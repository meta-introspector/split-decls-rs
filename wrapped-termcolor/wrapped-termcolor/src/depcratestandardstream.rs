// Generated macro for StandardStream (struct)
macro_rules! DepcrateStandardStream {
() => {
// Module: crate
// Provides: {"StandardStream"}
// Dependencies: {}
# [doc = " Satisfies `io::Write` and `WriteColor`, and supports optional coloring"] # [doc = " to either of the standard output streams, stdout and stderr."] # [derive (Debug)] pub struct StandardStream { wtr : LossyStandardStream < WriterInner < IoStandardStream > > , }
};
}
