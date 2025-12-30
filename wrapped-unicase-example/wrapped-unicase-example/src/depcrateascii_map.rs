// Generated macro for ASCII_MAP (static)
macro_rules! DepcrateASCII_MAP {
() => {
// Module: crate
// Provides: {"ASCII_MAP"}
// Dependencies: {}
pub static ASCII_MAP : phf :: Map < Ascii < & 'static str > , isize > = phf_map ! (Ascii :: new ("Foo") => 0 , Ascii :: new ("Bar") => 1 ,) ;
};
}
