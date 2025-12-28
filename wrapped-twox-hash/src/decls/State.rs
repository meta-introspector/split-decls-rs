macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " Constructs [`Hasher`][] for multiple hasher instances."] # [derive (Clone)] pub struct State (u64) ;
    };
}

State!();