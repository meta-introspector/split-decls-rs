macro_rules! deps {
    () => {
        KeyDeserializer!();
        Value!();
        Error!();
        TableMapAccess!();
        ValueDeserializer!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: MapAccess < 'de > for TableMapAccess < 'de > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Self :: Error > where K : serde_core :: de :: DeserializeSeed < 'de > , { match self . iter . next () { Some ((k , v)) => { let key_span = k . span () ; let ret = seed . deserialize (super :: KeyDeserializer :: new (k . clone () . into_inner () , Some (key_span . clone ()) ,)) . map (Some) . map_err (| mut e : Self :: Error | { if e . span () . is_none () { e . set_span (Some (key_span)) ; } e }) ; self . value = Some ((k , v)) ; ret } None => Ok (None) , } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Self :: Error > where V : serde_core :: de :: DeserializeSeed < 'de > , { match self . value . take () { Some ((k , v)) => { let span = v . span () ; seed . deserialize (crate :: de :: ValueDeserializer :: with_parts (v . into_inner () , span . clone () ,)) . map_err (| mut e : Self :: Error | { if e . span () . is_none () { e . set_span (Some (span)) ; } e . add_key (k . into_inner () . into_owned ()) ; e }) } None => { panic ! ("no more values in next_value_seed, internal error in ValueDeserializer") } } } }
    };
}

impl_140!()