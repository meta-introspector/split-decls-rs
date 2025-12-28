macro_rules! deps {
    () => {
        ArraySeqAccess!();
        Error!();
        ValueDeserializer!();
        Value!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: SeqAccess < 'de > for ArraySeqAccess { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { match self . iter . next () { Some (v) => seed . deserialize (crate :: de :: ValueDeserializer :: new (v)) . map (Some) , None => Ok (None) , } } }
    };
}

impl_283!();