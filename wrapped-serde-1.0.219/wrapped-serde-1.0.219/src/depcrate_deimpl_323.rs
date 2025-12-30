// Generated macro for impl_323 (impl)
macro_rules! Depcrate_deimpl_323 {
() => {
// Module: crate::de
// Provides: {"impl_323"}
// Dependencies: {}
impl Display for OneOf { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self . names . len () { 0 => panic ! () , 1 => write ! (formatter , "`{}`" , self . names [0]) , 2 => write ! (formatter , "`{}` or `{}`" , self . names [0] , self . names [1]) , _ => { tri ! (formatter . write_str ("one of ")) ; for (i , alt) in self . names . iter () . enumerate () { if i > 0 { tri ! (formatter . write_str (", ")) ; } tri ! (write ! (formatter , "`{}`" , alt)) ; } Ok (()) } } } }
};
}
