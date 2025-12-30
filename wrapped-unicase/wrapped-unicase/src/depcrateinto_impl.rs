// Generated macro for into_impl (macro)
macro_rules! Depcrateinto_impl {
() => {
// Module: crate
// Provides: {"into_impl"}
// Dependencies: {}
macro_rules ! into_impl { ($ to : ty) => { impl <'a > Into <$ to > for UniCase <$ to > { fn into (self) -> $ to { self . into_inner () } } } ; }
};
}
