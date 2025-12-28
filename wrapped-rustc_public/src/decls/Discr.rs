macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! Discr {
    () => {
        deps!();
        pub struct Discr { pub val : u128 , pub ty : Ty , }
    };
}

Discr!();