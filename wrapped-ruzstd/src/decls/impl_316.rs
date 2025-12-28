macro_rules! deps {
    () => {
        SymbolStates!();
        State!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl SymbolStates { fn get (& self , idx : usize , max_idx : usize) -> & State { let start_search_at = (idx * self . states . len ()) / max_idx ; self . states [start_search_at ..] . iter () . find (| state | state . contains (idx)) . unwrap () } }
    };
}

impl_316!();