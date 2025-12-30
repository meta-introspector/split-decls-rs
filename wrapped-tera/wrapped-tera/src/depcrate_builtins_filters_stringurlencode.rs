// Generated macro for urlencode (function)
macro_rules! Depcrate_builtins_filters_stringurlencode {
() => {
// Module: crate::builtins::filters::string
// Provides: {"urlencode"}
// Dependencies: {}
# [doc = " Percent-encodes reserved URI characters"] # [cfg (feature = "urlencode")] pub fn urlencode (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("urlencode" , "value" , String , value) ; let encoded = percent_encode (s . as_bytes () , PYTHON_ENCODE_SET) . to_string () ; Ok (Value :: String (encoded)) }
};
}
