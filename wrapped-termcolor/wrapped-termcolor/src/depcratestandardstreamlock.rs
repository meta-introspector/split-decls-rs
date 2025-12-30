// Generated macro for StandardStreamLock (struct)
macro_rules! DepcrateStandardStreamLock {
() => {
// Module: crate
// Provides: {"StandardStreamLock"}
// Dependencies: {}
# [doc = " `StandardStreamLock` is a locked reference to a `StandardStream`."] # [doc = ""] # [doc = " This implements the `io::Write` and `WriteColor` traits, and is constructed"] # [doc = " via the `Write::lock` method."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the corresponding"] # [doc = " `StandardStream`."] # [derive (Debug)] pub struct StandardStreamLock < 'a > { wtr : LossyStandardStream < WriterInnerLock < 'a , IoStandardStreamLock < 'a > > > , }
};
}
