// Generated macro for sizeof_impl (macro)
macro_rules! Depcratesizeof_impl {
() => {
// Module: crate
// Provides: {"sizeof_impl"}
// Dependencies: {}
# [doc = " Implements [`TypeSize`] for multiple types based on the return value of [`core::mem::size_of`]."] # [macro_export] macro_rules ! sizeof_impl { ($ ($ ty : ty) ,*) => { $ (impl TypeSize for $ ty { }) * } ; }
};
}
