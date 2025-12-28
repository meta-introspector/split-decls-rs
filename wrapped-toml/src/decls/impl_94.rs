macro_rules! deps {
    () => {
        MapDeserializer!();
        Value!();
        Error!();
        MapEnumDeserializer!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'de > de :: EnumAccess < 'de > for MapDeserializer { type Error = crate :: de :: Error ; type Variant = MapEnumDeserializer ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : de :: DeserializeSeed < 'de > , { use de :: Error ; let (key , value) = match self . iter . next () { Some (pair) => pair , None => { return Err (Error :: custom ("expected table with exactly 1 entry, found empty table" ,)) ; } } ; let val = seed . deserialize (key . into_deserializer ()) ? ; let variant = MapEnumDeserializer :: new (value) ; Ok ((val , variant)) } }
    };
}

impl_94!();