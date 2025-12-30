// Generated macro for impl_807 (impl)
macro_rules! Depcrate_listsimpl_807 {
() => {
// Module: crate::lists
// Provides: {"impl_807"}
// Dependencies: {}
impl < 'a , T , I , F1 , F2 , F3 > Iterator for ListItems < 'a , I , F1 , F2 , F3 > where I : Iterator < Item = T > , F1 : Fn (& T) -> BytePos , F2 : Fn (& T) -> BytePos , F3 : Fn (& T) -> RewriteResult , { type Item = ListItem ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| item | { let pre_snippet = self . snippet_provider . span_to_snippet (mk_sp (self . prev_span_end , (self . get_lo) (& item))) . unwrap_or ("") ; let (pre_comment , pre_comment_style) = extract_pre_comment (pre_snippet) ; let next_start = match self . inner . peek () { Some (next_item) => (self . get_lo) (next_item) , None => self . next_span_start , } ; let post_snippet = self . snippet_provider . span_to_snippet (mk_sp ((self . get_hi) (& item) , next_start)) . unwrap_or ("") ; let is_last = self . inner . peek () . is_none () ; let comment_end = get_comment_end (post_snippet , self . separator , self . terminator , is_last) ; let new_lines = has_extra_newline (post_snippet , comment_end) ; let post_comment = extract_post_comment (post_snippet , comment_end , self . separator , is_last) ; self . prev_span_end = (self . get_hi) (& item) + BytePos (comment_end as u32) ; ListItem { pre_comment , pre_comment_style , item : if self . inner . peek () . is_none () && self . leave_last { Err (RewriteError :: SkipFormatting) } else { (self . get_item_string) (& item) } , post_comment , new_lines , } }) } }
};
}
