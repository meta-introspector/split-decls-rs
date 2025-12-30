// Generated macro for ErrorKind (enum)
macro_rules! DepcrateErrorKind {
() => {
// Module: crate
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " The various errors that can occur during formatting. Note that not all of"] # [doc = " these can currently be propagated to clients."] # [derive (Error , Debug)] pub enum ErrorKind { # [doc = " Line has exceeded character limit (found, maximum)."] # [error ("line formatted, but exceeded maximum width \
         (maximum: {1} (see `max_width` option), found: {0})")] LineOverflow (usize , usize) , # [doc = " Line ends in whitespace."] # [error ("left behind trailing whitespace")] TrailingWhitespace , # [doc = " Used deprecated skip attribute."] # [error ("`rustfmt_skip` is deprecated; use `rustfmt::skip`")] DeprecatedAttr , # [doc = " Used a rustfmt:: attribute other than skip or skip::macros."] # [error ("invalid attribute")] BadAttr , # [doc = " An io error during reading or writing."] # [error ("io error: {0}")] IoError (io :: Error) , # [doc = " Error during module resolution."] # [error ("{0}")] ModuleResolutionError (# [from] ModuleResolutionError) , # [doc = " Parse error occurred when parsing the input."] # [error ("parse error")] ParseError , # [doc = " The user mandated a version and the current version of Rustfmt does not"] # [doc = " satisfy that requirement."] # [error ("version mismatch")] VersionMismatch , # [doc = " If we had formatted the given node, then we would have lost a comment."] # [error ("not formatted because a comment would be lost")] LostComment , # [doc = " Invalid glob pattern in `ignore` configuration option."] # [error ("Invalid glob pattern found in ignore list: {0}")] InvalidGlobPattern (ignore :: Error) , }
};
}
