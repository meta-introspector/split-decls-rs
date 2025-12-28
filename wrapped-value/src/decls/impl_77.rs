macro_rules! deps {
    () => {
        Name!();
        ConstValue!();
        Variables!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for Variables { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self (< Option < BTreeMap < Name , ConstValue > > > :: deserialize (deserializer) ? . unwrap_or_default () ,)) } }
    };
}

impl_77!()