// Generated macro for State (enum)
macro_rules! DepcrateState {
() => {
// Module: crate
// Provides: {"State"}
// Dependencies: {}
enum State { # [doc = " Within a delimiter."] Delimiter , # [doc = " After backslash, but before starting word."] Backslash , # [doc = " Within an unquoted word."] Unquoted , # [doc = " After backslash in an unquoted word."] UnquotedBackslash , # [doc = " Within a single quoted word."] SingleQuoted , # [doc = " Within a double quoted word."] DoubleQuoted , # [doc = " After backslash inside a double quoted word."] DoubleQuotedBackslash , # [doc = " Inside a comment."] Comment , }
};
}
