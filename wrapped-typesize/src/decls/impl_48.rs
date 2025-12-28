macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl TypeSize for serde_json :: Value { fn extra_size (& self) -> usize { match self { Self :: Null => 0 , Self :: Bool (value) => value . extra_size () , Self :: Number (value) => value . extra_size () , Self :: String (value) => value . extra_size () , Self :: Array (value) => value . extra_size () , Self :: Object (value) => value . extra_size () , } } }
    };
}

impl_48!();