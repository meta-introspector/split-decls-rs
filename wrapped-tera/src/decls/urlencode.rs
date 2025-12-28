macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! urlencode {
    () => {
        deps!();
        # [doc = " Percent-encodes reserved URI characters"] # [cfg (feature = "urlencode")] pub fn urlencode (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("urlencode" , "value" , String , value) ; let encoded = percent_encode (s . as_bytes () , PYTHON_ENCODE_SET) . to_string () ; Ok (Value :: String (encoded)) }
    };
}

urlencode!();