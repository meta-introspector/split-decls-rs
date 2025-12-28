macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! GetValue {
    () => {
        deps!();
        pub trait GetValue : Ord + Sized + Clone { fn get_value (val : & Value) -> Result < Self > ; }
    };
}

GetValue!();