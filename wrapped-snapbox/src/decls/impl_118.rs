macro_rules! deps {
    () => {
        Data!();
        ToDebug!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < D : std :: fmt :: Debug > ToDebug for D { fn to_debug (& self) -> Data { Data :: text (format ! ("{self:#?}\n")) } }
    };
}

impl_118!()