// Generated macro for StandardStreamType (enum)
macro_rules! DepcrateStandardStreamType {
() => {
// Module: crate
// Provides: {"StandardStreamType"}
// Dependencies: {}
# [doc = " `std::io` implements `Stdout` and `Stderr` (and their `Lock` variants) as"] # [doc = " separate types, which makes it difficult to abstract over them. We use"] # [doc = " some simple internal enum types to work around this."] enum StandardStreamType { Stdout , Stderr , StdoutBuffered , StderrBuffered , }
};
}
