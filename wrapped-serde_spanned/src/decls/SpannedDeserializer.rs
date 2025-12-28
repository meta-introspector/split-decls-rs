macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! SpannedDeserializer {
    () => {
        deps!();
        # [doc = " Deserializer / format support for emitting [`Spanned`]"] pub struct SpannedDeserializer < 'de , T , E > where T : serde_core :: de :: IntoDeserializer < 'de , E > , E : serde_core :: de :: Error , { start : Option < usize > , end : Option < usize > , value : Option < T > , _lifetime : core :: marker :: PhantomData < & 'de () > , _error : core :: marker :: PhantomData < E > , }
    };
}

SpannedDeserializer!();