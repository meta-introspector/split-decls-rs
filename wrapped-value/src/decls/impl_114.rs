macro_rules! deps {
    () => {
        Value!();
        ConstValue!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl ConstValue { # [doc = " Convert this `ConstValue` into a `Value`."] # [must_use] pub fn into_value (self) -> Value { match self { Self :: Null => Value :: Null , Self :: Number (num) => Value :: Number (num) , Self :: String (s) => Value :: String (s) , Self :: Boolean (b) => Value :: Boolean (b) , Self :: Binary (bytes) => Value :: Binary (bytes) , Self :: Enum (v) => Value :: Enum (v) , Self :: List (items) => { Value :: List (items . into_iter () . map (ConstValue :: into_value) . collect ()) } Self :: Object (map) => Value :: Object (map . into_iter () . map (| (key , value) | (key , value . into_value ())) . collect () ,) , } } # [doc = " Attempt to convert the value into JSON. This is equivalent to the"] # [doc = " `TryFrom` implementation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if serialization fails (see enum docs for more info)."] pub fn into_json (self) -> serde_json :: Result < serde_json :: Value > { self . try_into () } # [doc = " Attempt to convert JSON into a value. This is equivalent to the"] # [doc = " `TryFrom` implementation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if deserialization fails (see enum docs for more info)."] pub fn from_json (json : serde_json :: Value) -> serde_json :: Result < Self > { json . try_into () } }
    };
}

impl_114!()