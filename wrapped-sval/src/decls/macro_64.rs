macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! macro_64 {
    () => {
        deps!();
        impl_value_forward ! ({ impl <'a , T : Value + ? Sized > Value for &'a T } => x => { ** x }) ;
    };
}

macro_64!();