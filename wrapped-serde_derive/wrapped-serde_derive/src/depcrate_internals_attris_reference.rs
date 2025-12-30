// Generated macro for is_reference (function)
macro_rules! Depcrate_internals_attris_reference {
() => {
// Module: crate::internals::attr
// Provides: {"is_reference"}
// Dependencies: {}
fn is_reference (ty : & syn :: Type , elem : fn (& syn :: Type) -> bool) -> bool { match ungroup (ty) { syn :: Type :: Reference (ty) => ty . mutability . is_none () && elem (& ty . elem) , _ => false , } }
};
}
