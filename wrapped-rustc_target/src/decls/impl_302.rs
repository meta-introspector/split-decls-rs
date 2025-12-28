macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < T : ToString , A : ToJson > ToJson for BTreeMap < T , A > { fn to_json (& self) -> Json { let mut d = Map :: new () ; for (key , value) in self { d . insert (key . to_string () , value . to_json ()) ; } Json :: Object (d) } }
    };
}

impl_302!();