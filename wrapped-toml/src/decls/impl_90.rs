macro_rules! deps {
    () => {
        SeqDeserializer!();
        Value!();
        Error!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'de > de :: SeqAccess < 'de > for SeqDeserializer { type Error = crate :: de :: Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , crate :: de :: Error > where T : de :: DeserializeSeed < 'de > , { match self . iter . next () { Some (value) => seed . deserialize (value) . map (Some) , None => Ok (None) , } } fn size_hint (& self) -> Option < usize > { match self . iter . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
    };
}

impl_90!()