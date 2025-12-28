macro_rules! item_if_cached {
    () => {
        macro_rules ! item_if_cached { ([] $ tokens : tt) => { } ; ([(cache) $ ($ rest : tt) *] { $ ($ tokens : tt) * }) => { $ ($ tokens) * } ; ([$ other : tt $ ($ modifiers : tt) *] $ tokens : tt) => { item_if_cached ! { [$ ($ modifiers) *] $ tokens } } ; }
    };
}

item_if_cached!();