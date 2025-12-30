// Generated macro for alt_trait_impl (macro)
macro_rules! Depcrate_combinator_branchalt_trait_impl {
() => {
// Module: crate::combinator::branch
// Provides: {"alt_trait_impl"}
// Dependencies: {}
macro_rules ! alt_trait_impl (($ ($ id : ident) +) => (impl < I : Stream , Output , Error : ParserError < I >, $ ($ id : Parser < I , Output , Error >) ,+ > Alt < I , Output , Error > for ($ ($ id) ,+) { fn choice (& mut self , input : & mut I) -> Result < Output , Error > { let start = input . checkpoint () ; match self . 0 . parse_next (input) { Err (e) if e . is_backtrack () => alt_trait_inner ! (1 , self , input , start , e , $ ($ id) +) , res => res , } } }) ;) ;
};
}
