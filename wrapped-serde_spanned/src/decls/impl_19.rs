macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , T > serde_core :: de :: Deserialize < 'de > for Spanned < T > where T : serde_core :: de :: Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { struct SpannedVisitor < T > (:: core :: marker :: PhantomData < T >) ; impl < 'de , T > serde_core :: de :: Visitor < 'de > for SpannedVisitor < T > where T : serde_core :: de :: Deserialize < 'de > , { type Value = Spanned < T > ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { formatter . write_str ("a spanned value") } fn visit_map < V > (self , mut visitor : V) -> Result < Spanned < T > , V :: Error > where V : serde_core :: de :: MapAccess < 'de > , { let mut start : Option < usize > = None ; let mut end : Option < usize > = None ; let mut value : Option < T > = None ; while let Some (key) = visitor . next_key () ? { match key { START_FIELD => { if start . is_some () { return Err (serde_core :: de :: Error :: duplicate_field (START_FIELD)) ; } start = Some (visitor . next_value () ?) ; } END_FIELD => { if end . is_some () { return Err (serde_core :: de :: Error :: duplicate_field (END_FIELD)) ; } end = Some (visitor . next_value () ?) ; } VALUE_FIELD => { if value . is_some () { return Err (serde_core :: de :: Error :: duplicate_field (VALUE_FIELD)) ; } value = Some (visitor . next_value () ?) ; } field => { return Err (serde_core :: de :: Error :: unknown_field (field , & [START_FIELD , END_FIELD , VALUE_FIELD] ,)) ; } } } match (start , end , value) { (Some (start) , Some (end) , Some (value)) => Ok (Spanned { span : start .. end , value , }) , (None , _ , _) => Err (serde_core :: de :: Error :: missing_field (START_FIELD)) , (_ , None , _) => Err (serde_core :: de :: Error :: missing_field (END_FIELD)) , (_ , _ , None) => Err (serde_core :: de :: Error :: missing_field (VALUE_FIELD)) , } } } static FIELDS : [& str ; 3] = [START_FIELD , END_FIELD , VALUE_FIELD] ; let visitor = SpannedVisitor (:: core :: marker :: PhantomData) ; deserializer . deserialize_struct (NAME , & FIELDS , visitor) } }
    };
}

impl_19!();