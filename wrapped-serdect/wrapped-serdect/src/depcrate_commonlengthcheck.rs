// Generated macro for LengthCheck (trait)
macro_rules! Depcrate_commonLengthCheck {
() => {
// Module: crate::common
// Provides: {"LengthCheck"}
// Dependencies: {}
pub (crate) trait LengthCheck { fn length_check (buffer_length : usize , data_length : usize) -> bool ; fn expecting (formatter : & mut fmt :: Formatter < '_ > , data_type : & str , data_length : usize ,) -> fmt :: Result ; }
};
}
