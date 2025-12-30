// Generated macro for impl_1096 (impl)
macro_rules! Depcrate_combinator_implsimpl_1096 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1096"}
// Dependencies: {}
impl < F , G , I , O , E , E2 > Parser < I , O , E2 > for MapErr < F , G , I , O , E , E2 > where F : Parser < I , O , E > , G : FnMut (E) -> E2 , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O , E2 > { match self . parser . parse_next (i) { Err (e) => Err ((self . map) (e)) , Ok (o) => Ok (o) , } } }
};
}
