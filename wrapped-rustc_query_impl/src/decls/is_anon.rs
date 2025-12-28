macro_rules! is_anon {
    () => {
        macro_rules ! is_anon { ([]) => { { false } } ; ([(anon) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { is_anon ! ([$ ($ modifiers) *]) } ; }
    };
}

is_anon!()