// Generated macro for impl_229 (impl)
macro_rules! Depcrate_text_inlineimpl_229 {
() => {
// Module: crate::text::inline
// Provides: {"impl_229"}
// Dependencies: {}
impl < 's , T : DiffableStr + ? Sized > From < Change < & 's T > > for InlineChange < 's , T > { fn from (change : Change < & 's T >) -> InlineChange < 's , T > { InlineChange { tag : change . tag () , old_index : change . old_index () , new_index : change . new_index () , values : vec ! [(false , change . value ())] , } } }
};
}
