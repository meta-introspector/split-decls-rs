// Generated macro for dec_int (function)
macro_rules! Depcrate_asciidec_int {
() => {
// Module: crate::ascii
// Provides: {"dec_int"}
// Dependencies: {}
# [doc = " Decode a decimal signed integer (e.g. [`i32`])"] # [doc = ""] # [doc = " *Complete version*: can parse until the end of input."] # [doc = ""] # [doc = " *[Partial version][crate::_topic::partial]*: Will return `Err(winnow::error::ErrMode::Incomplete(_))` if there's not enough input data."] # [doc = ""] # [doc = " # Effective Signature"] # [doc = ""] # [doc = " Assuming you are parsing a `&str` [Stream] into an `i32`:"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;;"] # [doc = " pub fn dec_int(input: &mut &str) -> ModalResult<i32>"] # [doc = " # {"] # [doc = " #     winnow::ascii::dec_int.parse_next(input)"] # [doc = " # }"] # [doc = " ```"] # [doc (alias = "i8")] # [doc (alias = "i16")] # [doc (alias = "i32")] # [doc (alias = "i64")] # [doc (alias = "i128")] pub fn dec_int < Input , Output , Error > (input : & mut Input) -> Result < Output , Error > where Input : StreamIsPartial + Stream , < Input as Stream > :: Slice : AsBStr , < Input as Stream > :: Token : AsChar + Clone , Output : Int , Error : ParserError < Input > , { trace ("dec_int" , move | input : & mut Input | { let sign = opt (dispatch ! { any . map (AsChar :: as_char) ; '+' => empty . value (true) , '-' => empty . value (false) , _ => fail , }) ; alt (((sign , one_of ('1' ..= '9') , digit0) . void () , one_of ('0') . void ())) . take () . verify_map (| s : < Input as Stream > :: Slice | { let s = s . as_bstr () ; let s = unsafe { core :: str :: from_utf8_unchecked (s) } ; Output :: try_from_dec_int (s) }) . parse_next (input) }) . parse_next (input) }
};
}
