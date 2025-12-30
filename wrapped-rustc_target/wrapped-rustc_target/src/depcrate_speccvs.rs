// Generated macro for cvs (macro)
macro_rules! Depcrate_speccvs {
() => {
// Module: crate::spec
// Provides: {"cvs"}
// Dependencies: {}
# [doc = " Cow-Vec-Str: Cow<'static, [Cow<'static, str>]>"] macro_rules ! cvs { () => { :: std :: borrow :: Cow :: Borrowed (& []) } ; ($ ($ x : expr) ,+ $ (,) ?) => { :: std :: borrow :: Cow :: Borrowed (& [$ (:: std :: borrow :: Cow :: Borrowed ($ x) ,) *]) } ; }
};
}
