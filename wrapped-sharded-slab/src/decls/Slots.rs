macro_rules! deps {
    () => {
        Slot!();
    };
}

macro_rules! Slots {
    () => {
        deps!();
        type Slots < T , C > = Box < [Slot < T , C >] > ;
    };
}

Slots!()