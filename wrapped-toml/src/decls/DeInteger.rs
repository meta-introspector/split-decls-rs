macro_rules! deps {
    () => {
        DeString!();
    };
}

macro_rules! DeInteger {
    () => {
        deps!();
        # [doc = " Represents a TOML integer"] # [derive (Clone , Debug)] pub struct DeInteger < 'i > { pub (crate) inner : DeString < 'i > , pub (crate) radix : u32 , }
    };
}

DeInteger!()