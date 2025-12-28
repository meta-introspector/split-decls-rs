macro_rules! deps {
    () => {
        CursorPosition!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl CursorPosition { fn block_entry (block : BasicBlock) -> CursorPosition { CursorPosition { block , curr_effect_index : None } } }
    };
}

impl_33!()