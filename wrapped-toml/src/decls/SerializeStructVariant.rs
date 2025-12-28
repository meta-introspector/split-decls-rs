macro_rules! deps {
    () => {
        SerializeTable!();
    };
}

macro_rules! SerializeStructVariant {
    () => {
        deps!();
        pub struct SerializeStructVariant < 'd > { inner : SerializeTable < 'd > , }
    };
}

SerializeStructVariant!()