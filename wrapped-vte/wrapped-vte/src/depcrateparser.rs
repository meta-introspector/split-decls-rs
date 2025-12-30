// Generated macro for Parser (struct)
macro_rules! DepcrateParser {
() => {
// Module: crate
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " Parser for raw _VTE_ protocol which delegates actions to a [`Perform`]"] # [doc = ""] # [doc = " [`Perform`]: trait.Perform.html"] # [doc = ""] # [doc = " Generic over the value for the size of the raw Operating System Command"] # [doc = " buffer. Only used when the `std` feature is not enabled."] # [derive (Default)] pub struct Parser < const OSC_RAW_BUF_SIZE : usize = MAX_OSC_RAW > { state : State , intermediates : [u8 ; MAX_INTERMEDIATES] , intermediate_idx : usize , params : Params , param : u16 , # [cfg (not (feature = "std"))] osc_raw : ArrayVec < u8 , OSC_RAW_BUF_SIZE > , # [cfg (feature = "std")] osc_raw : Vec < u8 > , osc_params : [(usize , usize) ; MAX_OSC_PARAMS] , osc_num_params : usize , ignoring : bool , partial_utf8 : [u8 ; 4] , partial_utf8_len : usize , }
};
}
