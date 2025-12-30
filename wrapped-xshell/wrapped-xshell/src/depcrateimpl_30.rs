// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl fmt :: Display for Cmd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . secret { return write ! (f , "<secret>") ; } write ! (f , "{}" , self . prog . as_path () . display ()) ? ; for arg in & self . args { let arg = arg . to_string_lossy () ; if arg . chars () . any (| it | it . is_ascii_whitespace ()) { write ! (f , " \"{}\"" , arg . escape_default ()) ? } else { write ! (f , " {}" , arg) ? } ; } Ok (()) } }
};
}
