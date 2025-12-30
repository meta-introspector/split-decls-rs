// Generated macro for impl_53 (impl)
macro_rules! Depcrate_sliceimpl_53 {
() => {
// Module: crate::slice
// Provides: {"impl_53"}
// Dependencies: {}
impl LengthCheck for UpperBound { fn length_check (buffer_length : usize , data_length : usize) -> bool { buffer_length >= data_length } fn expecting (formatter : & mut fmt :: Formatter < '_ > , data_type : & str , data_length : usize ,) -> fmt :: Result { write ! (formatter , "{data_type} with a maximum length of {data_length}") } }
};
}
