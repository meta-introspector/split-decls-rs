macro_rules! deps {
    () => {
        VisitMap!();
        DatetimeOrTable!();
        DatetimeFromString!();
        Datetime!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'de > VisitMap < 'de > { # [doc = " Determine the type of the map by deserializing it"] pub fn next_key_seed < V : serde_core :: de :: MapAccess < 'de > > (visitor : & mut V ,) -> Result < Option < Self > , V :: Error > { let mut key = None ; let Some (()) = visitor . next_key_seed (DatetimeOrTable :: new (& mut key)) ? else { return Ok (None) ; } ; let result = if let Some (key) = key { VisitMap :: Key (key) } else { let date : crate :: datetime :: DatetimeFromString = visitor . next_value () ? ; VisitMap :: Datetime (date . value) } ; Ok (Some (result)) } }
    };
}

impl_46!();