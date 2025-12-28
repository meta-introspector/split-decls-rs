macro_rules! deps {
    () => {
        DatetimeFromString!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de > serde_core :: de :: Deserialize < 'de > for DatetimeFromString { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { struct Visitor ; impl serde_core :: de :: Visitor < '_ > for Visitor { type Value = DatetimeFromString ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("string containing a datetime") } fn visit_str < E > (self , s : & str) -> Result < DatetimeFromString , E > where E : serde_core :: de :: Error , { match s . parse () { Ok (date) => Ok (DatetimeFromString { value : date }) , Err (e) => Err (serde_core :: de :: Error :: custom (e)) , } } } deserializer . deserialize_str (Visitor) } }
    };
}

impl_39!()