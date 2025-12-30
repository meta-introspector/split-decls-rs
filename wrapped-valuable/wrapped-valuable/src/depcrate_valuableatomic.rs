// Generated macro for atomic (macro)
macro_rules! Depcrate_valuableatomic {
() => {
// Module: crate::valuable
// Provides: {"atomic"}
// Dependencies: {}
# [cfg (not (valuable_no_atomic))] macro_rules ! atomic { ($ ($ (# [$ attrs : meta]) * $ variant : ident ($ ty : ident) ,) *) => { $ ($ (# [$ attrs]) * impl Valuable for core :: sync :: atomic ::$ ty { fn as_value (& self) -> Value <'_ > { Value ::$ variant (self . load (core :: sync :: atomic :: Ordering :: SeqCst)) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (self . as_value ()) ; } }) * } ; }
};
}
