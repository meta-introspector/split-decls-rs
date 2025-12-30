// Generated macro for MultiLookup (struct)
macro_rules! Depcrate_text_inlineMultiLookup {
() => {
// Module: crate::text::inline
// Provides: {"MultiLookup"}
// Dependencies: {}
struct MultiLookup < 'bufs , 's , T : DiffableStr + ? Sized > { strings : & 'bufs [& 's T] , seqs : Vec < (& 's T , usize , usize) > , }
};
}
