macro_rules! deps {
    () => {
        Durability!();
        Revision!();
        Stamp!();
    };
}

macro_rules! stamp {
    () => {
        deps!();
        pub fn stamp (revision : Revision , durability : Durability) -> Stamp { Stamp { durability , changed_at : revision , } }
    };
}

stamp!();