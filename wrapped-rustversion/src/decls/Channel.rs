macro_rules! deps {
    () => {
        Date!();
    };
}

macro_rules! Channel {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq)] pub enum Channel { Stable , Beta , Nightly (Date) , Dev , }
    };
}

Channel!()