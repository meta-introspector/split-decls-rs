macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! TypeAndMut {
    () => {
        deps!();
        pub struct TypeAndMut { pub ty : Ty , pub mutability : Mutability , }
    };
}

TypeAndMut!();