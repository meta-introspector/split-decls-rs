macro_rules! deps {
    () => {
        Bridge!();
    };
}

macro_rules! Prov {
    () => {
        deps!();
        pub trait Prov < B : Bridge > { fn new (aid : B :: AllocId) -> Self ; }
    };
}

Prov!()