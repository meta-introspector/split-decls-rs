// Generated macro for from_iter_method (macro)
macro_rules! Depcrate_tendrilfrom_iter_method {
() => {
// Module: crate::tendril
// Provides: {"from_iter_method"}
// Dependencies: {}
macro_rules ! from_iter_method { ($ ty : ty) => { # [inline] fn from_iter < I > (iterable : I) -> Self where I : IntoIterator < Item = $ ty > { let mut output = Self :: new () ; output . extend (iterable) ; output } } }
};
}
