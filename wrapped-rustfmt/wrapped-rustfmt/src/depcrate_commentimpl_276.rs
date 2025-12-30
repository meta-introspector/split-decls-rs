// Generated macro for impl_276 (impl)
macro_rules! Depcrate_commentimpl_276 {
() => {
// Module: crate::comment
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'a > Iterator for CommentReducer < 'a > { type Item = char ; fn next (& mut self) -> Option < Self :: Item > { loop { let mut c = self . iter . next () ? ; if self . is_block && self . at_start_line { while c . is_whitespace () { c = self . iter . next () ? ; } if c == '*' { c = self . iter . next () ? ; } } else if c == '\n' { self . at_start_line = true ; } if ! c . is_whitespace () { return Some (c) ; } } } }
};
}
