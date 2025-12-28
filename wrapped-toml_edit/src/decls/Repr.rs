macro_rules! deps {
    () => {
        RawString!();
    };
}

macro_rules! Repr {
    () => {
        deps!();
        # [doc = " A TOML [`Value`][crate::Value] encoded as a `&str`"] # [derive (Eq , PartialEq , Clone , Hash)] pub struct Repr { raw_value : RawString , }
    };
}

Repr!();