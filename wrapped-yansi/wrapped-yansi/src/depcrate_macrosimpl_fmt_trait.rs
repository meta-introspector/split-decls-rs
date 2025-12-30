// Generated macro for impl_fmt_trait (macro)
macro_rules! Depcrate_macrosimpl_fmt_trait {
() => {
// Module: crate::macros
// Provides: {"impl_fmt_trait"}
// Dependencies: {}
macro_rules ! impl_fmt_trait { ($ F : path , $ f : literal <$ G : ident > $ T : ty => $ s : ident .$ v : ident ($ V : ty)) => { impl <$ G : $ F > $ F for $ T { fn fmt (&$ s , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { $ s . fmt_args (&<$ V >:: fmt , f , format_args ! ($ f , $ s .$ v)) } } } ; }
};
}
