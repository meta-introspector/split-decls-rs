// Generated macro for DecodeError (struct)
macro_rules! Depcrate_responseDecodeError {
() => {
// Module: crate::response
// Provides: {"DecodeError"}
// Dependencies: {}
# [doc = " An error occurred while decoding a response body to a string."] # [doc = ""] # [doc = " The error carries the encoding that was used to attempt to decode the body, and the raw byte"] # [doc = " contents of the body. This can be used to treat un-decodable bodies specially or to implement a"] # [doc = " fallback parsing strategy."] # [derive (Clone)] pub struct DecodeError { # [doc = " The name of the encoding that was used to try to decode the input."] pub encoding : String , # [doc = " The input data as bytes."] pub data : Vec < u8 > , }
};
}
