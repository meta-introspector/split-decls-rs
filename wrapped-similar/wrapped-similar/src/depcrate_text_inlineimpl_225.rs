// Generated macro for impl_225 (impl)
macro_rules! Depcrate_text_inlineimpl_225 {
() => {
// Module: crate::text::inline
// Provides: {"impl_225"}
// Dependencies: {}
impl < T : DiffableStr + ? Sized > Index < usize > for MultiLookup < '_ , '_ , T > { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { self . seqs [index] . 0 } }
};
}
