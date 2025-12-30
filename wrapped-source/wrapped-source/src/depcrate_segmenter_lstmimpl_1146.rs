// Generated macro for impl_1146 (impl)
macro_rules! Depcrate_segmenter_lstmimpl_1146 {
() => {
// Module: crate::segmenter::lstm
// Provides: {"impl_1146"}
// Dependencies: {}
impl RawLstmMatrix { fn to_ndarray1 (& self) -> Result < Array1 < f32 > , DataError > { if self . dim . len () == 1 { Ok (Array :: from_vec (self . data . clone ())) } else { Err (DIMENSION_MISMATCH_ERROR) } } fn to_ndarray2 (& self) -> Result < Array2 < f32 > , DataError > { let [d0 , d1] = * < & [usize ; 2] > :: try_from (self . dim . as_slice ()) . map_err (| _ | DIMENSION_MISMATCH_ERROR) ? ; Array :: from_shape_vec ((d0 , d1) , self . data . clone ()) . map_err (| _ | DIMENSION_MISMATCH_ERROR) } }
};
}
