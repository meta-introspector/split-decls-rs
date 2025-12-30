// Generated macro for slice_ty (function)
macro_rules! Depcrate_utilslice_ty {
() => {
// Module: crate::util
// Provides: {"slice_ty"}
// Dependencies: {}
# [doc = " From `T` create `[T]`."] pub (crate) fn slice_ty (t : syn :: Type) -> syn :: Type { syn :: TypeSlice { bracket_token : Default :: default () , elem : Box :: new (t) , } . into () }
};
}
