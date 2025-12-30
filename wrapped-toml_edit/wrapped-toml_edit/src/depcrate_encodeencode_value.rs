// Generated macro for encode_value (function)
macro_rules! Depcrate_encodeencode_value {
() => {
// Module: crate::encode
// Provides: {"encode_value"}
// Dependencies: {}
pub (crate) fn encode_value (this : & Value , buf : & mut dyn Write , input : Option < & str > , default_decor : (& str , & str) ,) -> Result { match this { Value :: String (repr) => encode_formatted (repr , buf , input , default_decor) , Value :: Integer (repr) => encode_formatted (repr , buf , input , default_decor) , Value :: Float (repr) => encode_formatted (repr , buf , input , default_decor) , Value :: Boolean (repr) => encode_formatted (repr , buf , input , default_decor) , Value :: Datetime (repr) => encode_formatted (repr , buf , input , default_decor) , Value :: Array (array) => encode_array (array , buf , input , default_decor) , Value :: InlineTable (table) => encode_table (table , buf , input , default_decor) , } }
};
}
