// Generated macro for impl_22 (impl)
macro_rules! Depcrate_boundimpl_22 {
() => {
// Module: crate::bound
// Provides: {"impl_22"}
// Dependencies: {}
impl PartialOrd < Bound > for Version { fn partial_cmp (& self , rhs : & Bound) -> Option < Ordering > { match rhs { Bound :: Nightly (date) => match self . channel { Stable | Beta => Some (Ordering :: Less) , Nightly (nightly) => Some (nightly . cmp (date)) , Dev => Some (Ordering :: Greater) , } , Bound :: Stable (release) => { let version = (self . minor , self . patch) ; let bound = (release . minor , release . patch . unwrap_or (0)) ; Some (version . cmp (& bound)) } } } }
};
}
