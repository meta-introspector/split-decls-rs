macro_rules! feedable {
    () => {
        macro_rules ! feedable { ([]) => { { false } } ; ([(feedable) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { feedable ! ([$ ($ modifiers) *]) } ; }
    };
}

feedable!()