// Generated macro for Span (struct)
macro_rules! Depcrate_treeSpan {
() => {
// Module: crate::tree
// Provides: {"Span"}
// Dependencies: {}
# [doc = " An internal node in the log tree carrying information about a Tracing span."] # [derive (Clone , Debug)] # [cfg_attr (feature = "serde" , derive (Serialize))] pub struct Span { # [doc = " Shared fields between events and spans."] # [cfg_attr (feature = "serde" , serde (flatten))] pub (crate) shared : Shared , # [doc = " The name of the span."] pub (crate) name : & 'static str , # [doc = " The total duration the span was open for."] # [cfg_attr (feature = "serde" , serde (rename = "nanos_total" , serialize_with = "ser::nanos"))] pub (crate) total_duration : Duration , # [doc = " The total duration inner spans were open for."] # [cfg_attr (feature = "serde" , serde (rename = "nanos_nested" , serialize_with = "ser::nanos"))] pub (crate) inner_duration : Duration , # [doc = " Events and spans collected while the span was open."] pub (crate) nodes : Vec < Tree > , # [doc = " This span is only displayed *if* there are child nodes in the tree. Else it"] # [doc = " will NOT be rendered."] # [cfg (feature = "defer")] pub (crate) defer_unless_children_attached : bool , }
};
}
