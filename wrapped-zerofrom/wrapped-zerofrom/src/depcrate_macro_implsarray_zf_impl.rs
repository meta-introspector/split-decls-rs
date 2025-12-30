// Generated macro for array_zf_impl (macro)
macro_rules! Depcrate_macro_implsarray_zf_impl {
() => {
// Module: crate::macro_impls
// Provides: {"array_zf_impl"}
// Dependencies: {}
macro_rules ! array_zf_impl { ($ n : expr ; $ ($ i : expr) ,+) => { impl <'a , C , T : ZeroFrom <'a , C >> ZeroFrom <'a , [C ; $ n] > for [T ; $ n] { fn zero_from (this : &'a [C ; $ n]) -> Self { [$ (< T as ZeroFrom < C >>:: zero_from (& this [$ i])) ,+] } } } }
};
}
