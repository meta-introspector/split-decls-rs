macro_rules! deps {
    () => {
        Datetime!();
        Date!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de > serde_core :: de :: Deserialize < 'de > for Date { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { match Datetime :: deserialize (deserializer) ? { Datetime { date : Some (date) , time : None , offset : None , } => Ok (date) , datetime => Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: Other (datetime . type_name ()) , & Self :: type_name () ,)) , } } }
    };
}

impl_34!()