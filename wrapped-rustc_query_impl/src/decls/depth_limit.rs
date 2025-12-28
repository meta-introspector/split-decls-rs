macro_rules! depth_limit {
    () => {
        macro_rules ! depth_limit { ([]) => { { false } } ; ([(depth_limit) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { depth_limit ! ([$ ($ modifiers) *]) } ; }
    };
}

depth_limit!();