// Generated macro for fetch_generics (function)
macro_rules! Depcratefetch_generics {
() => {
// Module: crate
// Provides: {"fetch_generics"}
// Dependencies: {}
fn fetch_generics < 'a > (set : & [bool] , generics : & 'a Generics) -> Vec < & 'a Ident > { let mut tys = vec ! [] ; for (& seen , param) in set . iter () . zip (generics . params . iter ()) { if seen { if let GenericParam :: Type (tparam) = param { tys . push (& tparam . ident) ; } } } tys }
};
}
