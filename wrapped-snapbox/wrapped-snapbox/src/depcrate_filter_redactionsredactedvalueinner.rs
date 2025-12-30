// Generated macro for RedactedValueInner (enum)
macro_rules! Depcrate_filter_redactionsRedactedValueInner {
() => {
// Module: crate::filter::redactions
// Provides: {"RedactedValueInner"}
// Dependencies: {}
# [derive (Clone , Debug)] enum RedactedValueInner { Str (& 'static str) , String (String) , Path { native : String , normalized : String , } , # [cfg (feature = "regex")] Regex (regex :: Regex) , }
};
}
