macro_rules! deps {
    () => {
        Value!();
        Error!();
        TableEnumDeserializer!();
        TableMapAccess!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'de > serde_core :: de :: EnumAccess < 'de > for TableMapAccess { type Error = Error ; type Variant = super :: TableEnumDeserializer ; fn variant_seed < V > (mut self , seed : V) -> Result < (< V as serde_core :: de :: DeserializeSeed < 'de > > :: Value , < Self as serde_core :: de :: EnumAccess < 'de > > :: Variant) , < Self as serde_core :: de :: EnumAccess < 'de > > :: Error > where V : serde_core :: de :: DeserializeSeed < 'de > , { let (key , value) = match self . iter . next () { Some (pair) => pair , None => { return Err (Error :: custom ("expected table with exactly 1 entry, found empty table" , self . span ,)) ; } } ; let val = seed . deserialize (key . into_deserializer ()) . map_err (| mut e : Self :: Error | { if e . span () . is_none () { e . set_span (key . span ()) ; } e }) ? ; let variant = super :: TableEnumDeserializer :: new (value) ; Ok ((val , variant)) } }
    };
}

impl_310!();