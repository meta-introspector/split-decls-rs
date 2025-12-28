macro_rules! deps {
    () => {
        Spanned!();
        SpannedDeserializer!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'de , T , E > SpannedDeserializer < 'de , T , E > where T : serde_core :: de :: IntoDeserializer < 'de , E > , E : serde_core :: de :: Error , { # [doc = " Create a deserializer to emit [`Spanned`]"] pub fn new (value : T , span : core :: ops :: Range < usize >) -> Self { Self { start : Some (span . start) , end : Some (span . end) , value : Some (value) , _lifetime : Default :: default () , _error : Default :: default () , } } }
    };
}

impl_24!();