macro_rules! deps {
    () => {
        MapValueSerializer!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl < 'd > MapValueSerializer < 'd > { fn new (is_none : & 'd mut bool) -> Self { Self { is_none } } }
    };
}

impl_371!()