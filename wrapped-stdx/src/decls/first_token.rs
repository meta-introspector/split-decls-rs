macro_rules! first_token {
    () => {
        macro_rules ! first_token { ($ first : tt $ ($ rest : tt) *) => { $ first } ; }
    };
}

first_token!();