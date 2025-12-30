// Generated macro for Change (struct)
macro_rules! Depcrate_typesChange {
() => {
// Module: crate::types
// Provides: {"Change"}
// Dependencies: {}
# [doc = " Represents the expanded [`DiffOp`] change."] # [doc = ""] # [doc = " This type is returned from [`DiffOp::iter_changes`] and"] # [doc = " [`TextDiff::iter_changes`](crate::text::TextDiff::iter_changes)."] # [doc = ""] # [doc = " It exists so that it's more convenient to work with textual differences as"] # [doc = " the underlying [`DiffOp`] encodes a group of changes."] # [doc = ""] # [doc = " This type has additional methods that are only available for types"] # [doc = " implementing [`DiffableStr`](crate::text::DiffableStr)."] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy , Ord , PartialOrd)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize))] pub struct Change < T > { pub (crate) tag : ChangeTag , pub (crate) old_index : Option < usize > , pub (crate) new_index : Option < usize > , pub (crate) value : T , }
};
}
