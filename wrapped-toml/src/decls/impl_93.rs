macro_rules! deps {
    () => {
        Error!();
        MapDeserializer!();
        Value!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'de > de :: MapAccess < 'de > for MapDeserializer { type Error = crate :: de :: Error ; fn next_key_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , crate :: de :: Error > where T : de :: DeserializeSeed < 'de > , { match self . iter . next () { Some ((key , value)) => { self . value = Some ((key . clone () , value)) ; seed . deserialize (Value :: String (key)) . map (Some) } None => Ok (None) , } } fn next_value_seed < T > (& mut self , seed : T) -> Result < T :: Value , crate :: de :: Error > where T : de :: DeserializeSeed < 'de > , { let (key , res) = match self . value . take () { Some ((key , value)) => (key , seed . deserialize (value)) , None => return Err (de :: Error :: custom ("value is missing")) , } ; res . map_err (| mut error | { error . add_key (key) ; error }) } fn size_hint (& self) -> Option < usize > { match self . iter . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
    };
}

impl_93!()