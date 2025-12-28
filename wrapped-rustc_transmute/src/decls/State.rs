macro_rules! deps {
    () => {
        Dfa!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " The states in a [`Dfa`] represent byte offsets."] # [derive (Hash , Eq , PartialEq , PartialOrd , Ord , Copy , Clone)] pub (crate) struct State (pub (crate) u32) ;
    };
}

State!();