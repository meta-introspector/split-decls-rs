// Generated macro for product_pat (macro)
macro_rules! Depcrate_genericproduct_pat {
() => {
// Module: crate::generic
// Provides: {"product_pat"}
// Dependencies: {}
macro_rules ! product_pat { ($ H : pat) => { Product ($ H , ()) } ; ($ H : pat , $ ($ T : pat) ,*) => { Product ($ H , product_pat ! ($ ($ T) ,*)) } ; }
};
}
