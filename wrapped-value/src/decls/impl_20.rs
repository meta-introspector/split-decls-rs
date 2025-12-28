macro_rules! deps {
    () => {
        MapDeserializer!();
        Value!();
        MapKeyDeserializer!();
        DeserializerError!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'de > MapAccess < 'de > for MapDeserializer { type Error = DeserializerError ; fn next_key_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , DeserializerError > where T : DeserializeSeed < 'de > , { match self . iter . next () { Some ((key , value)) => { self . value = Some (value) ; let key_de = MapKeyDeserializer { key } ; seed . deserialize (key_de) . map (Some) } None => Ok (None) , } } # [inline] fn next_value_seed < T > (& mut self , seed : T) -> Result < T :: Value , DeserializerError > where T : DeserializeSeed < 'de > , { match self . value . take () { Some (value) => seed . deserialize (value) , None => Err (serde :: de :: Error :: custom ("value is missing")) , } } # [inline] fn size_hint (& self) -> Option < usize > { match self . iter . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
    };
}

impl_20!();