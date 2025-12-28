macro_rules! deps {
    () => {
        DeString!();
    };
}

macro_rules! DeFloat {
    () => {
        deps!();
        # [doc = " Represents a TOML integer"] # [derive (Clone , Debug)] pub struct DeFloat < 'i > { pub (crate) inner : DeString < 'i > , }
    };
}

DeFloat!()