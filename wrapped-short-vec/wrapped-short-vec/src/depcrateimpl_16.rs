// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'de , T > Visitor < 'de > for ShortVecVisitor < T > where T : Deserialize < 'de > , { type Value = Vec < T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a Vec with a multi-byte length") } fn visit_seq < A > (self , mut seq : A) -> Result < Vec < T > , A :: Error > where A : SeqAccess < 'de > , { let short_len : ShortU16 = seq . next_element () ? . ok_or_else (| | de :: Error :: invalid_length (0 , & self)) ? ; let len = short_len . 0 as usize ; let mut result = Vec :: with_capacity (len) ; for i in 0 .. len { let elem = seq . next_element () ? . ok_or_else (| | de :: Error :: invalid_length (i , & self)) ? ; result . push (elem) ; } Ok (result) } }
};
}
