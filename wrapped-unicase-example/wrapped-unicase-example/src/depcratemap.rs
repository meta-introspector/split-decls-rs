// Generated macro for MAP (static)
macro_rules! DepcrateMAP {
() => {
// Module: crate
// Provides: {"MAP"}
// Dependencies: {}
pub static MAP : phf :: Map < UniCase < & 'static str > , isize > = phf_map ! (UniCase :: ascii ("Foo") => 0 , UniCase :: unicode ("Bar") => 1 ,) ;
};
}
