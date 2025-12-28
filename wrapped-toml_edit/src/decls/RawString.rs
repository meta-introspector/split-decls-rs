macro_rules! deps {
    () => {
        RawStringInner!();
    };
}

macro_rules! RawString {
    () => {
        deps!();
        # [doc = " Opaque string storage for raw TOML; internal to `toml_edit`"] # [derive (PartialEq , Eq , Clone , Hash)] pub struct RawString (RawStringInner) ;
    };
}

RawString!();