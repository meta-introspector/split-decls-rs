// Generated macro for impl_11 (impl)
macro_rules! Depcrate_arrayimpl_11 {
() => {
// Module: crate::array
// Provides: {"impl_11"}
// Dependencies: {}
impl LengthCheck for ExactLength { fn length_check (buffer_length : usize , data_length : usize) -> bool { buffer_length == data_length } fn expecting (formatter : & mut fmt :: Formatter < '_ > , data_type : & str , data_length : usize ,) -> fmt :: Result { write ! (formatter , "{data_type} of length {data_length}") } }
};
}
