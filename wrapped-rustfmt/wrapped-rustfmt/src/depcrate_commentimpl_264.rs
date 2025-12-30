// Generated macro for impl_264 (impl)
macro_rules! Depcrate_commentimpl_264 {
() => {
// Module: crate::comment
// Provides: {"impl_264"}
// Dependencies: {}
impl < 'a > Iterator for LineClasses < 'a > { type Item = (FullCodeCharKind , String) ; fn next (& mut self) -> Option < Self :: Item > { self . base . peek () ? ; let mut line = String :: new () ; let start_kind = match self . base . peek () { Some ((kind , _)) => * kind , None => unreachable ! () , } ; for (kind , c) in self . base . by_ref () { self . kind = kind ; if c == '\n' { self . kind = match (start_kind , kind) { (FullCodeCharKind :: Normal , FullCodeCharKind :: InString) => { FullCodeCharKind :: StartString } (FullCodeCharKind :: InString , FullCodeCharKind :: Normal) => { FullCodeCharKind :: EndString } (FullCodeCharKind :: InComment , FullCodeCharKind :: InStringCommented) => { FullCodeCharKind :: StartStringCommented } (FullCodeCharKind :: InStringCommented , FullCodeCharKind :: InComment) => { FullCodeCharKind :: EndStringCommented } _ => kind , } ; break ; } line . push (c) ; } if line . ends_with ('\r') { line . pop () ; } Some ((self . kind , line)) } }
};
}
