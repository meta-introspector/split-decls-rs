// Generated macro for decode_body (function)
macro_rules! Depcrate_responsedecode_body {
() => {
// Module: crate::response
// Provides: {"decode_body"}
// Dependencies: {}
# [doc = " Decode a response body as the given content type."] # [doc = ""] # [doc = " This always makes a copy. (It could be optimized to avoid the copy if the encoding is utf-8.)"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If an unsupported encoding is requested, or the body does not conform to the requested"] # [doc = " encoding, this function returns an `std::io::Error` of kind `std::io::ErrorKind::InvalidData`,"] # [doc = " carrying a `DecodeError` struct."] # [cfg (all (feature = "encoding" , target_arch = "wasm32"))] fn decode_body (mut bytes : Vec < u8 > , content_encoding : Option < & str >) -> Result < String , Error > { use web_sys :: TextDecoder ; let content_encoding = content_encoding . unwrap_or ("utf-8") . to_ascii_lowercase () ; if is_utf8_encoding (& content_encoding) { return String :: from_utf8 (bytes) . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err) . into ()) ; } let decoder = TextDecoder :: new_with_label (& content_encoding) . unwrap () ; Ok (decoder . decode_with_u8_array (& mut bytes) . map_err (| _ | { let err = DecodeError { encoding : content_encoding . to_string () , data : bytes , } ; io :: Error :: new (io :: ErrorKind :: InvalidData , err) }) ?) }
};
}
