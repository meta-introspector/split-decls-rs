// Generated macro for itemize_list (function)
macro_rules! Depcrate_listsitemize_list {
() => {
// Module: crate::lists
// Provides: {"itemize_list"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] pub (crate) fn itemize_list < 'a , T , I , F1 , F2 , F3 > (snippet_provider : & 'a SnippetProvider , inner : I , terminator : & 'a str , separator : & 'a str , get_lo : F1 , get_hi : F2 , get_item_string : F3 , prev_span_end : BytePos , next_span_start : BytePos , leave_last : bool ,) -> ListItems < 'a , I , F1 , F2 , F3 > where I : Iterator < Item = T > , F1 : Fn (& T) -> BytePos , F2 : Fn (& T) -> BytePos , F3 : Fn (& T) -> RewriteResult , { ListItems { snippet_provider , inner : inner . peekable () , get_lo , get_hi , get_item_string , prev_span_end , next_span_start , terminator , separator , leave_last , } }
};
}
