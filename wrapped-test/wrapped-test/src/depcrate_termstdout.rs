// Generated macro for stdout (function)
macro_rules! Depcrate_termstdout {
() => {
// Module: crate::term
// Provides: {"stdout"}
// Dependencies: {}
# [cfg (windows)] # [doc = " Returns a Terminal wrapping stdout, or None if a terminal couldn't be"] # [doc = " opened."] pub (crate) fn stdout () -> Option < Box < StdoutTerminal > > { TerminfoTerminal :: new (io :: stdout ()) . map (| t | Box :: new (t) as Box < StdoutTerminal >) . or_else (| | Some (Box :: new (WinConsole :: new (io :: stdout ())) as Box < StdoutTerminal >)) }
};
}
