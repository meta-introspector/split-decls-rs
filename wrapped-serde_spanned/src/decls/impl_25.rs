macro_rules! deps {
    () => {
        SpannedDeserializer!();
        Spanned!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'de , T , E > serde_core :: de :: MapAccess < 'de > for SpannedDeserializer < 'de , T , E > where T : serde_core :: de :: IntoDeserializer < 'de , E > , E : serde_core :: de :: Error , { type Error = E ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Self :: Error > where K : serde_core :: de :: DeserializeSeed < 'de > , { if self . start . is_some () { seed . deserialize (BorrowedStrDeserializer :: new (Spanned :: < T > :: START_FIELD)) . map (Some) } else if self . end . is_some () { seed . deserialize (BorrowedStrDeserializer :: new (Spanned :: < T > :: END_FIELD)) . map (Some) } else if self . value . is_some () { seed . deserialize (BorrowedStrDeserializer :: new (Spanned :: < T > :: VALUE_FIELD)) . map (Some) } else { Ok (None) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Self :: Error > where V : serde_core :: de :: DeserializeSeed < 'de > , { if let Some (start) = self . start . take () { seed . deserialize (start . into_deserializer ()) } else if let Some (end) = self . end . take () { seed . deserialize (end . into_deserializer ()) } else if let Some (value) = self . value . take () { seed . deserialize (value . into_deserializer ()) } else { panic ! ("next_value_seed called before next_key_seed") } } }
    };
}

impl_25!();