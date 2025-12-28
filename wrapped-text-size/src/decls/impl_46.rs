macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for TextRange { # [allow (clippy :: nonminimal_bool)] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let (start , end) = Deserialize :: deserialize (deserializer) ? ; if ! (start <= end) { return Err (de :: Error :: custom (format ! ("invalid range: {:?}..{:?}" , start , end))) ; } Ok (TextRange :: new (start , end)) } }
    };
}

impl_46!();