// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl From < ItemFn > for MaybeItemFn { fn from (ItemFn { attrs , vis , sig , block , } : ItemFn ,) -> Self { let (outer_attrs , inner_attrs) = attrs . into_iter () . partition (| attr | attr . style == syn :: AttrStyle :: Outer) ; let mut block_tokens = TokenStream :: new () ; block_tokens . append_all (block . stmts) ; Self { outer_attrs , inner_attrs , vis , sig , brace_token : block . brace_token , block : block_tokens , } } }
};
}
