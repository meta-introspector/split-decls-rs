macro_rules! deps {
    () => {
        DefaultConfig!();
        Slot!();
        Config!();
    };
}

macro_rules! Guard {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Guard < T , C : cfg :: Config = cfg :: DefaultConfig > { slot : ptr :: NonNull < Slot < T , C > > , }
    };
}

Guard!()