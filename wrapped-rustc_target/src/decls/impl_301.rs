macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < 'a , A : ToJson > ToJson for Cow < 'a , [A] > where [A] : ToOwned , { fn to_json (& self) -> Json { Json :: Array (self . iter () . map (| elt | elt . to_json ()) . collect ()) } }
    };
}

impl_301!()