// Generated macro for ValueRepr (trait)
macro_rules! Depcrate_reprValueRepr {
() => {
// Module: crate::repr
// Provides: {"ValueRepr"}
// Dependencies: {}
pub trait ValueRepr : crate :: private :: Sealed { # [doc = " The TOML representation of the value"] # [cfg (feature = "display")] fn to_repr (& self) -> Repr ; }
};
}
