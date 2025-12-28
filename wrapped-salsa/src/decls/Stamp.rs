macro_rules! deps {
    () => {
        Revision!();
        Durability!();
    };
}

macro_rules! Stamp {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub struct Stamp { pub durability : Durability , pub changed_at : Revision , }
    };
}

Stamp!();