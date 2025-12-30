// Generated macro for impl_21 (impl)
macro_rules! Depcrate_boundimpl_21 {
() => {
// Module: crate::bound
// Provides: {"impl_21"}
// Dependencies: {}
impl PartialEq < Bound > for Version { fn eq (& self , rhs : & Bound) -> bool { match rhs { Bound :: Nightly (date) => match self . channel { Stable | Beta | Dev => false , Nightly (nightly) => nightly == * date , } , Bound :: Stable (release) => { self . minor == release . minor && release . patch . map_or (true , | patch | self . patch == patch) } } } }
};
}
