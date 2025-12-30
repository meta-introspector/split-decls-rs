// Generated macro for SpanMap (struct)
macro_rules! Depcrate_mapSpanMap {
() => {
// Module: crate::map
// Provides: {"SpanMap"}
// Dependencies: {}
# [doc = " Maps absolute text ranges for the corresponding file to the relevant span data."] # [derive (Debug , PartialEq , Eq , Clone , Hash)] pub struct SpanMap < S > { # [doc = " The offset stored here is the *end* of the node."] spans : Vec < (TextSize , SpanData < S >) > , # [doc = " Index of the matched macro arm on successful expansion for declarative macros."] pub matched_arm : Option < u32 > , }
};
}
