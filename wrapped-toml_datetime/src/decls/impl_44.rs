macro_rules! deps {
    () => {
        DatetimeDeserializer!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'de , E > serde_core :: de :: MapAccess < 'de > for DatetimeDeserializer < E > where E : serde_core :: de :: Error , { type Error = E ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Self :: Error > where K : serde_core :: de :: DeserializeSeed < 'de > , { if self . date . is_some () { seed . deserialize (BorrowedStrDeserializer :: new (crate :: datetime :: FIELD)) . map (Some) } else { Ok (None) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Self :: Error > where V : serde_core :: de :: DeserializeSeed < 'de > , { if let Some (date) = self . date . take () { seed . deserialize (date . to_string () . into_deserializer ()) } else { panic ! ("next_value_seed called before next_key_seed") } } }
    };
}

impl_44!();