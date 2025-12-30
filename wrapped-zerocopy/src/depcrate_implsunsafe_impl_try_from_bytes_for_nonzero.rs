// Generated macro for unsafe_impl_try_from_bytes_for_nonzero (macro)
macro_rules! Depcrate_implsunsafe_impl_try_from_bytes_for_nonzero {
() => {
// Module: crate::impls
// Provides: {"unsafe_impl_try_from_bytes_for_nonzero"}
// Dependencies: {}
macro_rules ! unsafe_impl_try_from_bytes_for_nonzero { ($ ($ nonzero : ident [$ prim : ty]) ,*) => { $ (unsafe_impl ! (=> TryFromBytes for $ nonzero ; | n | { impl_size_eq ! ($ nonzero , Unalign <$ prim >) ; let n = n . transmute ::< Unalign <$ prim >, invariant :: Valid , _ > () ; $ nonzero :: new (n . read_unaligned () . into_inner ()) . is_some () }) ;) * } }
};
}
