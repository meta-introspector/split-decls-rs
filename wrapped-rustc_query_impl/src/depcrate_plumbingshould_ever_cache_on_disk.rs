// Generated macro for should_ever_cache_on_disk (macro)
macro_rules! Depcrate_plumbingshould_ever_cache_on_disk {
() => {
// Module: crate::plumbing
// Provides: {"should_ever_cache_on_disk"}
// Dependencies: {}
macro_rules ! should_ever_cache_on_disk { ([] $ yes : tt $ no : tt) => { { $ no } } ; ([(cache) $ ($ rest : tt) *] $ yes : tt $ no : tt) => { { $ yes } } ; ([$ other : tt $ ($ modifiers : tt) *] $ yes : tt $ no : tt) => { should_ever_cache_on_disk ! ([$ ($ modifiers) *] $ yes $ no) } ; }
};
}
