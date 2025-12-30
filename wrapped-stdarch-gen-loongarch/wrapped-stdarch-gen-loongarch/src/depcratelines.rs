// Generated macro for Lines (struct)
macro_rules! DepcrateLines {
() => {
// Module: crate
// Provides: {"Lines"}
// Dependencies: {}
# [doc = " Complete lines of generated source."] # [doc = ""] # [doc = " This enables common generation tasks to be factored out without precluding basic"] # [doc = " context-specific formatting."] # [doc = ""] # [doc = " The convention in this generator is to prefix (not suffix) lines with a newline, so the"] # [doc = " implementation of `std::fmt::Display` behaves in the same way."] struct Lines { indent : usize , lines : Vec < String > , }
};
}
