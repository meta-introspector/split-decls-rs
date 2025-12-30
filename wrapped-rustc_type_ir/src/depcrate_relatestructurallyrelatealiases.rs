// Generated macro for StructurallyRelateAliases (enum)
macro_rules! Depcrate_relateStructurallyRelateAliases {
() => {
// Module: crate::relate
// Provides: {"StructurallyRelateAliases"}
// Dependencies: {}
# [doc = " Whether aliases should be related structurally or not. Used"] # [doc = " to adjust the behavior of generalization and combine."] # [doc = ""] # [doc = " This should always be `No` unless in a few special-cases when"] # [doc = " instantiating canonical responses and in the new solver. Each"] # [doc = " such case should have a comment explaining why it is used."] # [derive (Debug , Copy , Clone)] pub enum StructurallyRelateAliases { Yes , No , }
};
}
