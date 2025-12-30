// Generated macro for permutation_trait_impl (macro)
macro_rules! Depcrate_combinator_branchpermutation_trait_impl {
() => {
// Module: crate::combinator::branch
// Provides: {"permutation_trait_impl"}
// Dependencies: {}
macro_rules ! permutation_trait_impl (($ ($ name : ident $ ty : ident $ item : ident) ,+) => (impl < I : Stream , $ ($ ty) ,+ , Error : ParserError < I >, $ ($ name : Parser < I , $ ty , Error >) ,+ > Permutation < I , ($ ($ ty) ,+) , Error > for ($ ($ name) ,+) { fn permutation (& mut self , input : & mut I) -> Result < ($ ($ ty) ,+) , Error > { let mut res = ($ (Option ::<$ ty >:: None) ,+) ; loop { let mut err : Option < Error > = None ; let start = input . checkpoint () ; permutation_trait_inner ! (0 , self , input , start , res , err , $ ($ name) +) ; if let Some (err) = err { input . reset (& start) ; return Err (err . append (input , & start)) ; } match res { ($ (Some ($ item)) ,+) => return Ok (($ ($ item) ,+)) , _ => unreachable ! () , } } } }) ;) ;
};
}
