// Generated macro for NodeData (struct)
macro_rules! DepcrateNodeData {
() => {
// Module: crate
// Provides: {"NodeData"}
// Dependencies: {}
# [doc = " Data about a node."] # [derive (Debug)] pub struct NodeData { # [doc = " The name of the node."] # [doc = ""] # [doc = " In the rule `A = 'b' | 'c'`, this is `\"A\"`."] pub name : String , # [doc = " The rule for this node."] # [doc = ""] # [doc = " In the rule `A = 'b' | 'c'`, this represents `'b' | 'c'`."] pub rule : Rule , }
};
}
