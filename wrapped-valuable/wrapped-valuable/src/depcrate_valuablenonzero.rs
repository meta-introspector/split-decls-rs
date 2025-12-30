// Generated macro for nonzero (macro)
macro_rules! Depcrate_valuablenonzero {
() => {
// Module: crate::valuable
// Provides: {"nonzero"}
// Dependencies: {}
macro_rules ! nonzero { ($ ($ variant : ident ($ ty : ident) ,) *) => { $ (impl Valuable for core :: num ::$ ty { fn as_value (& self) -> Value <'_ > { Value ::$ variant (self . get ()) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_value (self . as_value ()) ; } }) * } ; }
};
}
