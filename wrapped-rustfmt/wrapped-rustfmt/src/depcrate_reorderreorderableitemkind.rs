// Generated macro for ReorderableItemKind (enum)
macro_rules! Depcrate_reorderReorderableItemKind {
() => {
// Module: crate::reorder
// Provides: {"ReorderableItemKind"}
// Dependencies: {}
# [doc = " A simplified version of `ast::ItemKind`."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] enum ReorderableItemKind { ExternCrate , Mod , Use , # [doc = " An item that cannot be reordered. Either has an unreorderable item kind"] # [doc = " or an `macro_use` attribute."] Other , }
};
}
