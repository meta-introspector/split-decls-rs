// Generated macro for impl_10 (impl)
macro_rules! Depcrate_displayimpl_10 {
() => {
// Module: crate::display
// Provides: {"impl_10"}
// Dependencies: {}
impl Debug for Version { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = formatter . debug_struct ("Version") ; debug . field ("major" , & self . major) . field ("minor" , & self . minor) . field ("patch" , & self . patch) ; if ! self . pre . is_empty () { debug . field ("pre" , & self . pre) ; } if ! self . build . is_empty () { debug . field ("build" , & self . build) ; } debug . finish () } }
};
}
