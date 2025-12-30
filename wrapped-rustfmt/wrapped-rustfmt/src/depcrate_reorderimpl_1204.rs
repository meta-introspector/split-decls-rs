// Generated macro for impl_1204 (impl)
macro_rules! Depcrate_reorderimpl_1204 {
() => {
// Module: crate::reorder
// Provides: {"impl_1204"}
// Dependencies: {}
impl ReorderableItemKind { fn from (item : & ast :: Item) -> Self { match item . kind { _ if contains_macro_use_attr (item) | contains_skip (& item . attrs) => { ReorderableItemKind :: Other } ast :: ItemKind :: ExternCrate (..) => ReorderableItemKind :: ExternCrate , ast :: ItemKind :: Mod (..) if is_mod_decl (item) => ReorderableItemKind :: Mod , ast :: ItemKind :: Use (..) => ReorderableItemKind :: Use , _ => ReorderableItemKind :: Other , } } fn is_same_item_kind (self , item : & ast :: Item) -> bool { ReorderableItemKind :: from (item) == self } fn is_reorderable (self , config : & Config) -> bool { match self { ReorderableItemKind :: ExternCrate => config . reorder_imports () , ReorderableItemKind :: Mod => config . reorder_modules () , ReorderableItemKind :: Use => config . reorder_imports () , ReorderableItemKind :: Other => false , } } fn is_regroupable (self , config : & Config) -> bool { match self { ReorderableItemKind :: ExternCrate | ReorderableItemKind :: Mod | ReorderableItemKind :: Other => false , ReorderableItemKind :: Use => config . group_imports () != GroupImportsTactic :: Preserve , } } fn in_group (self , config : & Config) -> bool { match self { ReorderableItemKind :: ExternCrate | ReorderableItemKind :: Mod => true , ReorderableItemKind :: Use => config . group_imports () == GroupImportsTactic :: Preserve , ReorderableItemKind :: Other => false , } } }
};
}
