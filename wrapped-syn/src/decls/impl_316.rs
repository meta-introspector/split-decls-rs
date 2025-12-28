macro_rules! deps {
    () => {
        TypeGenerics!();
        Turbofish!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        # [cfg (feature = "printing")] impl < 'a > TypeGenerics < 'a > { # [doc = " Turn a type's generics like `<X, Y>` into a turbofish like `::<X, Y>`."] pub fn as_turbofish (& self) -> Turbofish < 'a > { Turbofish (self . 0) } }
    };
}

impl_316!();