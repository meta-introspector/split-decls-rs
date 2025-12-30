// Generated macro for impl_248 (impl)
macro_rules! Depcrate_commentimpl_248 {
() => {
// Module: crate::comment
// Provides: {"impl_248"}
// Dependencies: {}
impl FindUncommented for str { fn find_uncommented (& self , pat : & str) -> Option < usize > { let mut needle_iter = pat . chars () ; for (kind , (i , b)) in CharClasses :: new (self . char_indices ()) { match needle_iter . next () { None => { return Some (i - pat . len ()) ; } Some (c) => match kind { FullCodeCharKind :: Normal | FullCodeCharKind :: InString if b == c => { } _ => { needle_iter = pat . chars () ; } } , } } match needle_iter . next () { Some (_) => None , None => Some (self . len () - pat . len ()) , } } fn find_last_uncommented (& self , pat : & str) -> Option < usize > { if let Some (left) = self . find_uncommented (pat) { let mut result = left ; while let Some (next) = self [(result + 1) ..] . find_last_uncommented (pat) { result += next + 1 ; } Some (result) } else { None } } }
};
}
