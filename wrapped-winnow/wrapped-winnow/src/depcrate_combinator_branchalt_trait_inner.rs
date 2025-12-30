// Generated macro for alt_trait_inner (macro)
macro_rules! Depcrate_combinator_branchalt_trait_inner {
() => {
// Module: crate::combinator::branch
// Provides: {"alt_trait_inner"}
// Dependencies: {}
macro_rules ! alt_trait_inner (($ it : tt , $ self : expr , $ input : expr , $ start : ident , $ err : expr , $ head : ident $ ($ id : ident) +) => ({ $ input . reset (&$ start) ; match $ self .$ it . parse_next ($ input) { Err (e) if e . is_backtrack () => { let err = $ err . or (e) ; succ ! ($ it , alt_trait_inner ! ($ self , $ input , $ start , err , $ ($ id) +)) } res => res , } }) ; ($ it : tt , $ self : expr , $ input : expr , $ start : ident , $ err : expr , $ head : ident) => ({ Err ($ err . append ($ input , &$ start)) }) ;) ;
};
}
