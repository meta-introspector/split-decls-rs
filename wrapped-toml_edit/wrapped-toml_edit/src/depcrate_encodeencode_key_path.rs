// Generated macro for encode_key_path (function)
macro_rules! Depcrate_encodeencode_key_path {
() => {
// Module: crate::encode
// Provides: {"encode_key_path"}
// Dependencies: {}
fn encode_key_path (this : & [Key] , mut buf : & mut dyn Write , input : Option < & str > , default_decor : (& str , & str) ,) -> Result { let leaf_decor = this . last () . expect ("always at least one key") . leaf_decor () ; for (i , key) in this . iter () . enumerate () { let dotted_decor = key . dotted_decor () ; let first = i == 0 ; let last = i + 1 == this . len () ; if first { leaf_decor . prefix_encode (buf , input , default_decor . 0) ? ; } else { buf . key_sep () ? ; dotted_decor . prefix_encode (buf , input , DEFAULT_KEY_PATH_DECOR . 0) ? ; } encode_key (key , buf , input) ? ; if last { leaf_decor . suffix_encode (buf , input , default_decor . 1) ? ; } else { dotted_decor . suffix_encode (buf , input , DEFAULT_KEY_PATH_DECOR . 1) ? ; } } Ok (()) }
};
}
