// Generated macro for product (macro)
macro_rules! Depcrate_genericproduct {
() => {
// Module: crate::generic
// Provides: {"product"}
// Dependencies: {}
macro_rules ! product { ($ H : expr) => { Product ($ H , ()) } ; ($ H : expr , $ ($ T : expr) ,*) => { Product ($ H , product ! ($ ($ T) ,*)) } ; }
};
}
