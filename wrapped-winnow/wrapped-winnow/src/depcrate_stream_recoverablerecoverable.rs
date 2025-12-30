// Generated macro for Recoverable (struct)
macro_rules! Depcrate_stream_recoverableRecoverable {
() => {
// Module: crate::stream::recoverable
// Provides: {"Recoverable"}
// Dependencies: {}
# [doc = " Allow recovering from parse errors, capturing them as the parser continues"] # [doc = ""] # [doc = " Generally, this will be used indirectly via"] # [doc = " [`RecoverableParser::recoverable_parse`][crate::RecoverableParser::recoverable_parse]."] # [derive (Clone , Debug)] pub struct Recoverable < I , E > where I : Stream , { input : I , errors : Vec < E > , is_recoverable : bool , }
};
}
