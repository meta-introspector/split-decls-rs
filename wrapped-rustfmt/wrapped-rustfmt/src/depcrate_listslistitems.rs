// Generated macro for ListItems (struct)
macro_rules! Depcrate_listsListItems {
() => {
// Module: crate::lists
// Provides: {"ListItems"}
// Dependencies: {}
pub (crate) struct ListItems < 'a , I , F1 , F2 , F3 > where I : Iterator , { snippet_provider : & 'a SnippetProvider , inner : Peekable < I > , get_lo : F1 , get_hi : F2 , get_item_string : F3 , prev_span_end : BytePos , next_span_start : BytePos , terminator : & 'a str , separator : & 'a str , leave_last : bool , }
};
}
