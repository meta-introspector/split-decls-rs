macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ValueNumber {
    () => {
        deps!();
        pub trait ValueNumber { fn to_number (& self) -> Result < f64 , () > ; }
    };
}

ValueNumber!();