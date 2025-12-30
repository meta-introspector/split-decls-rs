// Generated macro for simple_enum_error (macro)
macro_rules! Depcrate_parsersimple_enum_error {
() => {
// Module: crate::parser
// Provides: {"simple_enum_error"}
// Dependencies: {}
macro_rules ! simple_enum_error { ($ ($ name : ident => $ description : expr ,) +) => { # [doc = " Errors that can occur during parsing."] # [doc = ""] # [doc = " This may be extended in the future so exhaustive matching is"] # [doc = " forbidden."] # [derive (PartialEq , Eq , Clone , Copy , Debug)] # [non_exhaustive] pub enum ParseError { $ ($ name ,) + } impl fmt :: Display for ParseError { fn fmt (& self , fmt : & mut Formatter <'_ >) -> fmt :: Result { match * self { $ (ParseError ::$ name => fmt . write_str ($ description) ,) + } } } } }
};
}
