macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl TypeSize for simd_json :: OwnedValue { fn extra_size (& self) -> usize { match self { simd_json :: OwnedValue :: Static (value) => value . extra_size () , simd_json :: OwnedValue :: String (value) => value . extra_size () , simd_json :: OwnedValue :: Object (value) => value . extra_size () , simd_json :: OwnedValue :: Array (value) => value . extra_size () , } } }
    };
}

impl_52!();