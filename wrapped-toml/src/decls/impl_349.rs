macro_rules! deps {
    () => {
        Error!();
        MapValueSerializer!();
        SerializeTable!();
        SerializeMap!();
        KeySerializer!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < 'd > serde_core :: ser :: SerializeMap for SerializeTable < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_key < T > (& mut self , input : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { let mut encoded_key = String :: new () ; input . serialize (KeySerializer { dst : & mut encoded_key , }) ? ; self . key = Some (encoded_key) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { let encoded_key = self . key . take () . expect ("always called after `serialize_key`") ; let mut encoded_value = String :: new () ; let mut is_none = false ; let value_serializer = MapValueSerializer :: new (& mut encoded_value , & mut is_none , self . style) ; let res = value . serialize (value_serializer) ; match res { Ok (_) => { use core :: fmt :: Write as _ ; if self . seen_value { self . dst . val_sep () ? ; } self . seen_value = true ; self . dst . space () ? ; write ! (self . dst , "{encoded_key}") ? ; self . dst . space () ? ; self . dst . keyval_sep () ? ; self . dst . space () ? ; write ! (self . dst , "{encoded_value}") ? ; } Err (e) => { if ! (e == Error :: unsupported_none () && is_none) { return Err (e) ; } } } Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
    };
}

impl_349!();