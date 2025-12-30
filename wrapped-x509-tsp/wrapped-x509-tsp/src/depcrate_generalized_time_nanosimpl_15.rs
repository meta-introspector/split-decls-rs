// Generated macro for impl_15 (impl)
macro_rules! Depcrate_generalized_time_nanosimpl_15 {
() => {
// Module: crate::generalized_time_nanos
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for GeneralizedTimeNanos { type Error = der :: Error ; # [rustfmt :: skip] fn decode_value < R : Reader < 'a > > (reader : & mut R , header : Header) -> Result < Self > { let len = usize :: try_from (header . length ()) ? ; if ! (Self :: MIN_LENGTH ..= Self :: MAX_LENGTH) . contains (& len) { return Err (Self :: TAG . value_error () . into ()) ; } let mut bytes = [0u8 ; Self :: MAX_LENGTH] ; let data = reader . read_into (& mut bytes [.. len]) ? ; match data { [y1 , y2 , y3 , y4 , mon1 , mon2 , day1 , day2 , hour1 , hour2 , min1 , min2 , sec1 , sec2 , b'Z'] => decode_from_values ((* y1 , * y2 , * y3 , * y4) , (* mon1 , * mon2) , (* day1 , * day2) , (* hour1 , * hour2) , (* min1 , * min2) , (* sec1 , * sec2) , None) , [y1 , y2 , y3 , y4 , mon1 , mon2 , day1 , day2 , hour1 , hour2 , min1 , min2 , sec1 , sec2 , b'.' , fract @ .. , b'Z'] => decode_from_values ((* y1 , * y2 , * y3 , * y4) , (* mon1 , * mon2) , (* day1 , * day2) , (* hour1 , * hour2) , (* min1 , * min2) , (* sec1 , * sec2) , Some (fract)) , _ => Err (Self :: TAG . value_error () . into ()) , } } }
};
}
