macro_rules! deps {
    () => {
        SymbolStates!();
    };
}

macro_rules! FSETable {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct FSETable { # [doc = " Indexed by symbol"] pub (super) states : [SymbolStates ; 256] , # [doc = " Sum of all states.states.len()"] pub (crate) table_size : usize , }
    };
}

FSETable!();