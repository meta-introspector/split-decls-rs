// Generated macro for InlineChange (struct)
macro_rules! Depcrate_text_inlineInlineChange {
() => {
// Module: crate::text::inline
// Provides: {"InlineChange"}
// Dependencies: {}
# [doc = " Represents the expanded textual change with inline highlights."] # [doc = ""] # [doc = " This is like [`Change`] but with inline highlight info."] # [derive (Debug , PartialEq , Eq , Hash , Clone , Ord , PartialOrd)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize))] pub struct InlineChange < 's , T : DiffableStr + ? Sized > { tag : ChangeTag , old_index : Option < usize > , new_index : Option < usize > , values : Vec < (bool , & 's T) > , }
};
}
