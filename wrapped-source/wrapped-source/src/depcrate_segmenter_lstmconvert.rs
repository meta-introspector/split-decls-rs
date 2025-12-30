// Generated macro for convert (macro)
macro_rules! Depcrate_segmenter_lstmconvert {
() => {
// Module: crate::segmenter::lstm
// Provides: {"convert"}
// Dependencies: {}
macro_rules ! convert { ($ fn_name : ident , $ matrix_name : ident , $ generic : literal) => { fn $ fn_name (nd : ArrayBase < OwnedRepr < f32 >, Dim < [usize ; $ generic] >>,) -> Result <$ matrix_name <'static >, DataError > where Dim < [usize ; $ generic] >: Dimension , { let dims = < [u16 ; $ generic] >:: try_from (nd . shape () . iter () . copied () . map (u16 :: try_from) . collect ::< Result < Vec < u16 >, _ >> () . map_err (| _ | DataError :: custom ("LSTM bounds too big for u16")) ?,) . map_err (| _ | DIMENSION_MISMATCH_ERROR) ?; let data = nd . as_slice_memory_order () . ok_or_else (|| DataError :: custom ("ndarray matrix not in memory order")) ?; $ matrix_name :: from_parts (dims , ZeroVec :: alloc_from_slice (data)) } } ; }
};
}
