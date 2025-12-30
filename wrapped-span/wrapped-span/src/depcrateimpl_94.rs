// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
impl fmt :: Display for Span { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . anchor . file_id . file_id () . index () , f) ? ; f . write_char (':') ? ; write ! (f , "{:#?}" , self . anchor . ast_id) ? ; f . write_char ('@') ? ; fmt :: Debug :: fmt (& self . range , f) ? ; f . write_char ('#') ? ; self . ctx . fmt (f) } }
};
}
