macro_rules! deps {
    () => {
        ConstValue!();
        Name!();
        Value!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Value { # [doc = " Attempt to convert the value into a const value by using a function to"] # [doc = " get a variable."] pub fn into_const_with < E > (self , mut f : impl FnMut (Name) -> Result < ConstValue , E > ,) -> Result < ConstValue , E > { self . into_const_with_mut (& mut f) } fn into_const_with_mut < E > (self , f : & mut impl FnMut (Name) -> Result < ConstValue , E > ,) -> Result < ConstValue , E > { Ok (match self { Self :: Variable (name) => f (name) ? , Self :: Null => ConstValue :: Null , Self :: Number (num) => ConstValue :: Number (num) , Self :: String (s) => ConstValue :: String (s) , Self :: Boolean (b) => ConstValue :: Boolean (b) , Self :: Binary (v) => ConstValue :: Binary (v) , Self :: Enum (v) => ConstValue :: Enum (v) , Self :: List (items) => ConstValue :: List (items . into_iter () . map (| value | value . into_const_with_mut (f)) . collect :: < Result < _ , _ > > () ? ,) , Self :: Object (map) => ConstValue :: Object (map . into_iter () . map (| (key , value) | Ok ((key , value . into_const_with_mut (f) ?))) . collect :: < Result < _ , _ > > () ? ,) , }) } # [doc = " Attempt to convert the value into a const value."] # [doc = ""] # [doc = " Will fail if the value contains variables."] # [must_use] pub fn into_const (self) -> Option < ConstValue > { self . into_const_with (| _ | Err (())) . ok () } # [doc = " Attempt to convert the value into JSON. This is equivalent to the"] # [doc = " `TryFrom` implementation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if serialization fails (see enum docs for more info)."] pub fn into_json (self) -> serde_json :: Result < serde_json :: Value > { self . try_into () } # [doc = " Attempt to convert JSON into a value. This is equivalent to the"] # [doc = " `TryFrom` implementation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if deserialization fails (see enum docs for more info)."] pub fn from_json (json : serde_json :: Value) -> serde_json :: Result < Self > { json . try_into () } }
    };
}

impl_43!()