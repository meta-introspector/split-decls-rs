macro_rules! deps {
    () => {
        Extensions!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Extensions { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self (< Option < HashMap < _ , _ > > > :: deserialize (deserializer) ? . unwrap_or_default () ,)) } }
    };
}

impl_30!();