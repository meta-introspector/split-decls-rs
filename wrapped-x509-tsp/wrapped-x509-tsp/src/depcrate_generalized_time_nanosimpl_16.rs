// Generated macro for impl_16 (impl)
macro_rules! Depcrate_generalized_time_nanosimpl_16 {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"impl_16"}
// Dependencies: {}
impl EncodeValue for GeneralizedTimeNanos { fn value_len (& self) -> Result < Length > { let mut len = Self :: MIN_LENGTH ; if self . nanoseconds != 0 { for_each_digits_without_trailing_zeroes (self . nanoseconds , | _ | { len += 1 ; Ok (()) }) ? ; len += 1 ; } Length :: try_from (len) } fn encode_value (& self , writer : & mut impl Writer) -> Result < () > { let year_hi = u8 :: try_from (self . datetime . year () / 100) ? ; let year_lo = u8 :: try_from (self . datetime . year () % 100) ? ; encode_decimal (writer , Self :: TAG , year_hi) ? ; encode_decimal (writer , Self :: TAG , year_lo) ? ; encode_decimal (writer , Self :: TAG , self . datetime . month ()) ? ; encode_decimal (writer , Self :: TAG , self . datetime . day ()) ? ; encode_decimal (writer , Self :: TAG , self . datetime . hour ()) ? ; encode_decimal (writer , Self :: TAG , self . datetime . minutes ()) ? ; encode_decimal (writer , Self :: TAG , self . datetime . seconds ()) ? ; if self . nanoseconds != 0 { writer . write_byte (b'.') ? ; encode_fractional_secs (writer , Self :: TAG , self . nanoseconds) ? ; } writer . write_byte (b'Z') } }
};
}
