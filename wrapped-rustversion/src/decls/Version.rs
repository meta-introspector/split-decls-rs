macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! Version {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq)] pub struct Version { pub minor : u16 , pub patch : u16 , pub channel : Channel , }
    };
}

Version!()