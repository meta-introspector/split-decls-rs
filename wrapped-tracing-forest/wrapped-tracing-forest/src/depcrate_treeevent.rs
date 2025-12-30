// Generated macro for Event (struct)
macro_rules! Depcrate_treeEvent {
() => {
// Module: crate::tree
// Provides: {"Event"}
// Dependencies: {}
# [doc = " A leaf node in the log tree carrying information about a Tracing event."] # [derive (Clone , Debug)] # [cfg_attr (feature = "serde" , derive (Serialize))] pub struct Event { # [doc = " Shared fields between events and spans."] # [cfg_attr (feature = "serde" , serde (flatten))] pub (crate) shared : Shared , # [doc = " The message associated with the event."] pub (crate) message : Option < String > , # [doc = " The tag that the event was collected with."] pub (crate) tag : Option < Tag > , }
};
}
