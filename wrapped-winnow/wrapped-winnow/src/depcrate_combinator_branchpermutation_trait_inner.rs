// Generated macro for permutation_trait_inner (macro)
macro_rules! Depcrate_combinator_branchpermutation_trait_inner {
() => {
// Module: crate::combinator::branch
// Provides: {"permutation_trait_inner"}
// Dependencies: {}
macro_rules ! permutation_trait_inner (($ it : tt , $ self : expr , $ input : ident , $ start : ident , $ res : expr , $ err : expr , $ head : ident $ ($ id : ident) *) => (if $ res .$ it . is_none () { $ input . reset (&$ start) ; match $ self .$ it . parse_next ($ input) { Ok (o) => { $ res .$ it = Some (o) ; continue ; } Err (e) if e . is_backtrack () => { $ err = Some (match $ err { Some (err) => err . or (e) , None => e , }) ; } Err (e) => return Err (e) , } ; } succ ! ($ it , permutation_trait_inner ! ($ self , $ input , $ start , $ res , $ err , $ ($ id) *)) ;) ; ($ it : tt , $ self : expr , $ input : ident , $ start : ident , $ res : expr , $ err : expr ,) => () ;) ;
};
}
