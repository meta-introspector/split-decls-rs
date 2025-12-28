macro_rules! deps {
    () => {
        Error!();
        TableMapAccess!();
        KeyDeserializer!();
        TableEnumDeserializer!();
        Value!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: EnumAccess < 'de > for TableMapAccess < 'de > { type Error = Error ; type Variant = super :: TableEnumDeserializer < 'de > ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : serde_core :: de :: DeserializeSeed < 'de > , { let (key , value) = match self . iter . next () { Some (pair) => pair , None => { return Err (Error :: custom ("expected table with exactly 1 entry, found empty table" , Some (self . span) ,)) ; } } ; let key_span = key . span () ; let val = seed . deserialize (super :: KeyDeserializer :: new (key . into_inner () , Some (key_span . clone ()) ,)) . map_err (| mut e : Self :: Error | { if e . span () . is_none () { e . set_span (Some (key_span)) ; } e }) ? ; let value_span = value . span () ; let value = value . into_inner () ; let variant = super :: TableEnumDeserializer :: new (value , value_span) ; Ok ((val , variant)) } }
    };
}

impl_141!()