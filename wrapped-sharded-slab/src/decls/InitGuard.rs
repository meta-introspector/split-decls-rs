macro_rules! deps {
    () => {
        Config!();
        DefaultConfig!();
        Slot!();
    };
}

macro_rules! InitGuard {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct InitGuard < T , C : cfg :: Config = cfg :: DefaultConfig > { slot : ptr :: NonNull < Slot < T , C > > , curr_lifecycle : usize , released : bool , }
    };
}

InitGuard!()