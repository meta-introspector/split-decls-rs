macro_rules! deps {
    () => {
        KeyDeserializer!();
        TableMapAccess!();
        Value!();
        ValueDeserializer!();
        Error!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: MapAccess < 'de > for TableMapAccess { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < < K as serde_core :: de :: DeserializeSeed < 'de > > :: Value > , < Self as serde_core :: de :: MapAccess < 'de > > :: Error > where K : serde_core :: de :: DeserializeSeed < 'de > , { match self . iter . next () { Some ((k , v)) => { let key_span = k . span () ; let ret = seed . deserialize (super :: KeyDeserializer :: new (k . clone () , key_span . clone ())) . map (Some) . map_err (| mut e : Self :: Error | { if e . span () . is_none () { e . set_span (key_span) ; } e }) ; self . value = Some ((k , v)) ; ret } None => Ok (None) , } } fn next_value_seed < V > (& mut self , seed : V) -> Result < < V as serde_core :: de :: DeserializeSeed < 'de > > :: Value , < Self as serde_core :: de :: MapAccess < 'de > > :: Error > where V : serde_core :: de :: DeserializeSeed < 'de > , { match self . value . take () { Some ((k , v)) => { let span = v . span () . or_else (| | k . span ()) ; seed . deserialize (crate :: de :: ValueDeserializer :: new (v)) . map_err (| mut e : Self :: Error | { if e . span () . is_none () { e . set_span (span) ; } e . add_key (k . get () . to_owned ()) ; e }) } None => { panic ! ("no more values in next_value_seed, internal error in ValueDeserializer") } } } }
    };
}

impl_309!()