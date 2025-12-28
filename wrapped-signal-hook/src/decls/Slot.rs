macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! Slot {
    () => {
        deps!();
        # [doc (hidden)] # [derive (Default , Debug)] pub struct Slot (AtomicPtr < Channel < siginfo_t > >) ;
    };
}

Slot!();