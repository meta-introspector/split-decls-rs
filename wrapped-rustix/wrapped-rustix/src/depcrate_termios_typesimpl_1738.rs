// Generated macro for impl_1738 (impl)
macro_rules! Depcrate_termios_typesimpl_1738 {
() => {
// Module: crate::termios::types
// Provides: {"impl_1738"}
// Dependencies: {}
impl core :: fmt :: Debug for SpecialCodes { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "SpecialCodes {{") ? ; let mut first = true ; for i in 0 .. self . 0 . len () { if first { write ! (f , " ") ? ; } else { write ! (f , ", ") ? ; } first = false ; let index = SpecialCodeIndex (i) ; write ! (f , "{:?}: {:?}" , index , SpecialCode (self [index])) ? ; } if ! first { write ! (f , " ") ? ; } write ! (f , "}}") } }
};
}
