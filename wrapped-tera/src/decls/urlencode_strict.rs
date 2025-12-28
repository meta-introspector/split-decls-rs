macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! urlencode_strict {
    () => {
        deps!();
        # [doc = " Percent-encodes all non-alphanumeric characters"] # [cfg (feature = "urlencode")] pub fn urlencode_strict (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let s = try_get_value ! ("urlencode_strict" , "value" , String , value) ; let encoded = percent_encode (s . as_bytes () , NON_ALPHANUMERIC) . to_string () ; Ok (Value :: String (encoded)) }
    };
}

urlencode_strict!()