macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Name { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self (String :: deserialize (deserializer) ? . into_boxed_str () . into () ,)) } }
    };
}

impl_96!();