// Generated macro for replace_string_markers (function)
macro_rules! Depcrate_parserreplace_string_markers {
() => {
// Module: crate::parser
// Provides: {"replace_string_markers"}
// Dependencies: {}
# [doc = " Strings are delimited by double quotes, single quotes and backticks"] # [doc = " We need to remove those before putting them in the AST"] fn replace_string_markers (input : & str) -> String { match input . chars () . next () . unwrap () { '"' => input . replace ('"' , "") , '\'' => input . replace ('\'' , "") , '`' => input . replace ('`' , "") , _ => unreachable ! ("How did you even get there") , } }
};
}
