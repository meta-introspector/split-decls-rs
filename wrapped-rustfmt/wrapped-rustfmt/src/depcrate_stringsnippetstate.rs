// Generated macro for SnippetState (enum)
macro_rules! Depcrate_stringSnippetState {
() => {
// Module: crate::string
// Provides: {"SnippetState"}
// Dependencies: {}
# [doc = " Result of breaking a string so it fits in a line and the state it ended in."] # [doc = " The state informs about what to do with the snippet and how to continue the breaking process."] # [derive (Debug , PartialEq)] enum SnippetState { # [doc = " The input could not be broken and so rewriting the string is finished."] EndOfInput (String) , # [doc = " The input could be broken and the returned snippet should be ended with a"] # [doc = " `[StringFormat::line_end]`. The next snippet needs to be indented."] # [doc = ""] # [doc = " The returned string is the line to print out and the number is the length that got read in"] # [doc = " the text being rewritten. That length may be greater than the returned string if trailing"] # [doc = " whitespaces got trimmed."] LineEnd (String , usize) , # [doc = " The input could be broken but a newline is present that cannot be trimmed. The next snippet"] # [doc = " to be rewritten *could* use more width than what is specified by the given shape. For"] # [doc = " example with a multiline string, the next snippet does not need to be indented, allowing"] # [doc = " more characters to be fit within a line."] # [doc = ""] # [doc = " The returned string is the line to print out and the number is the length that got read in"] # [doc = " the text being rewritten."] EndWithLineFeed (String , usize) , }
};
}
