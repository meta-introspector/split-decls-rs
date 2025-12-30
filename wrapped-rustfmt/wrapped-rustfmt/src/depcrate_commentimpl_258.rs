// Generated macro for impl_258 (impl)
macro_rules! Depcrate_commentimpl_258 {
() => {
// Module: crate::comment
// Provides: {"impl_258"}
// Dependencies: {}
impl FullCodeCharKind { pub (crate) fn is_comment (self) -> bool { match self { FullCodeCharKind :: StartComment | FullCodeCharKind :: InComment | FullCodeCharKind :: EndComment | FullCodeCharKind :: StartStringCommented | FullCodeCharKind :: InStringCommented | FullCodeCharKind :: EndStringCommented => true , _ => false , } } # [doc = " Returns true if the character is inside a comment"] pub (crate) fn inside_comment (self) -> bool { match self { FullCodeCharKind :: InComment | FullCodeCharKind :: StartStringCommented | FullCodeCharKind :: InStringCommented | FullCodeCharKind :: EndStringCommented => true , _ => false , } } pub (crate) fn is_string (self) -> bool { self == FullCodeCharKind :: InString || self == FullCodeCharKind :: StartString } # [doc = " Returns true if the character is within a commented string"] pub (crate) fn is_commented_string (self) -> bool { self == FullCodeCharKind :: InStringCommented || self == FullCodeCharKind :: StartStringCommented } fn to_codecharkind (self) -> CodeCharKind { if self . is_comment () { CodeCharKind :: Comment } else { CodeCharKind :: Normal } } }
};
}
