macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! SymbolStates {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (super) struct SymbolStates { # [doc = " Sorted by baseline to allow easy lookup using an index"] pub (super) states : Vec < State > , pub (super) probability : i32 , }
    };
}

SymbolStates!()