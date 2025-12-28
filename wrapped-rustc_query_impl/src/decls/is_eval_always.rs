macro_rules! is_eval_always {
    () => {
        macro_rules ! is_eval_always { ([]) => { { false } } ; ([(eval_always) $ ($ rest : tt) *]) => { { true } } ; ([$ other : tt $ ($ modifiers : tt) *]) => { is_eval_always ! ([$ ($ modifiers) *]) } ; }
    };
}

is_eval_always!();