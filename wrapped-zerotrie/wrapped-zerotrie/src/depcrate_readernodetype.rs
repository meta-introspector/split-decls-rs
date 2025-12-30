// Generated macro for NodeType (enum)
macro_rules! Depcrate_readerNodeType {
() => {
// Module: crate::reader
// Provides: {"NodeType"}
// Dependencies: {}
# [doc = " The node type. See the module-level docs for more explanation of the four node types."] enum NodeType { # [doc = " An ASCII node. Contains a single literal ASCII byte and no varint."] Ascii , # [doc = " A span node. Contains a varint indicating how big the span is."] Span , # [doc = " A value node. Contains a varint representing the value."] Value , # [doc = " A branch node. Contains a varint of the number of output nodes, plus W in the high bits."] Branch , }
};
}
