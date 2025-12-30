// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'ast > Visit < 'ast > for BoundAccumulator < 'ast > { fn visit_path (& mut self , path : & 'ast syn :: Path) { if path . segments . len () != 1 { return ; } if let Some (segment) = path . segments . first () { for param in & self . generics . params { if let syn :: GenericParam :: Type (type_param) = param { if type_param . ident == segment . ident && ! self . params . contains (& segment . ident) { self . params . push (type_param . ident . clone ()) ; } } } } } }
};
}
