macro_rules! deps {
    () => {
        ToJson!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < A : ToJson > ToJson for Option < A > { fn to_json (& self) -> Json { match * self { None => Json :: Null , Some (ref value) => value . to_json () , } } }
    };
}

impl_303!();