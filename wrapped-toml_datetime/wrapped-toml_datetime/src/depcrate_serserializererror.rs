// Generated macro for SerializerError (enum)
macro_rules! Depcrate_serSerializerError {
() => {
// Module: crate::ser
// Provides: {"SerializerError"}
// Dependencies: {}
# [doc = " See [`DatetimeSerializer`]"] # [derive (Debug)] # [non_exhaustive] pub enum SerializerError { # [doc = " Unsupported datetime format"] InvalidFormat (crate :: DatetimeParseError) , # [doc = " Unsupported serialization protocol"] InvalidProtocol , }
};
}
