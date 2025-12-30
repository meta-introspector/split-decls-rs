// Generated macro for dec_uint (function)
macro_rules! Depcrate_asciidec_uint {
() => {
// Module: crate::ascii
// Provides: {"dec_uint"}
// Dependencies: {}
# [doc = " Decode a decimal unsigned integer (e.g. [`u32`])"] # [doc = ""] # [doc = " *Complete version*: can parse until the end of input."] # [doc = ""] # [doc = " *[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data."] # [doc = ""] # [doc = " # Effective Signature"] # [doc = ""] # [doc = " Assuming you are parsing a `&str` [Stream] into a `u32`:"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;;"] # [doc = " pub fn dec_uint(input: &mut &str) -> ModalResult<u32>"] # [doc = " # {"] # [doc = " #     winnow::ascii::dec_uint.parse_next(input)"] # [doc = " # }"] # [doc = " ```"] # [doc (alias = "u8")] # [doc (alias = "u16")] # [doc (alias = "u32")] # [doc (alias = "u64")] # [doc (alias = "u128")] pub fn dec_uint < Input , Output , Error > (input : & mut Input) -> Result < Output , Error > where Input : StreamIsPartial + Stream , < Input as Stream > :: Slice : AsBStr , < Input as Stream > :: Token : AsChar + Clone , Output : Uint , Error : ParserError < Input > , { trace ("dec_uint" , move | input : & mut Input | { alt (((one_of ('1' ..= '9') , digit0) . void () , one_of ('0') . void ())) . take () . verify_map (| s : < Input as Stream > :: Slice | { let s = s . as_bstr () ; let s = unsafe { core :: str :: from_utf8_unchecked (s) } ; Output :: try_from_dec_uint (s) }) . parse_next (input) }) . parse_next (input) }
};
}
