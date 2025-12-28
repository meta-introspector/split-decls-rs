macro_rules! deps {
    () => {
        Value!();
        Error!();
        ValueDeserializer!();
        ArraySeqAccess!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: SeqAccess < 'de > for ArraySeqAccess < 'de > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { match self . iter . next () { Some (v) => { let span = v . span () ; let v = v . into_inner () ; seed . deserialize (crate :: de :: ValueDeserializer :: with_parts (v , span)) . map (Some) } None => Ok (None) , } } }
    };
}

impl_123!()