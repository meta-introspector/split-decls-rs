// Generated macro for trace (function)
macro_rules! Depcrate_parser_debugtrace {
() => {
// Module: crate::parser::debug
// Provides: {"trace"}
// Dependencies: {}
pub (crate) fn trace (text : & str , style : anstyle :: Style) { # ! [allow (unexpected_cfgs)] let depth = DEBUG_DEPTH . depth () ; anstream :: eprintln ! ("{:depth$}{style}{text}{style:#}" , "") ; }
};
}
