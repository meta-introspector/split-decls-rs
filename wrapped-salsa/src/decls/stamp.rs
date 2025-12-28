macro_rules! deps {
    () => {
        Revision!();
        Durability!();
        Stamp!();
    };
}

macro_rules! stamp {
    () => {
        deps!();
        pub fn stamp (revision : Revision , durability : Durability) -> Stamp { Stamp { durability , changed_at : revision , } }
    };
}

stamp!()