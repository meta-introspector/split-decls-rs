// Generated macro for RTL_BALANCED_NODE_GET_PARENT_POINTER (function)
macro_rules! Depcrate_shared_ntdefRTL_BALANCED_NODE_GET_PARENT_POINTER {
() => {
// Module: crate::shared::ntdef
// Provides: {"RTL_BALANCED_NODE_GET_PARENT_POINTER"}
// Dependencies: {}
# [inline] pub unsafe fn RTL_BALANCED_NODE_GET_PARENT_POINTER (Node : PRTL_BALANCED_NODE ,) -> PRTL_BALANCED_NODE { ((* Node) . ParentValue & ! RTL_BALANCED_NODE_RESERVED_PARENT_MASK) as * mut RTL_BALANCED_NODE }
};
}
