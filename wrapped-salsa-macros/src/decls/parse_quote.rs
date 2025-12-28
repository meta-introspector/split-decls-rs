macro_rules! parse_quote {
    () => {
        macro_rules ! parse_quote { ($ ($ inp : tt) *) => { { let tt = quote ! { $ ($ inp) * } ; syn :: parse2 (tt . clone ()) . unwrap_or_else (| err | { panic ! ("failed to parse `{}` at {}:{}:{}: {}" , tt , file ! () , line ! () , column ! () , err) }) } } }
    };
}

parse_quote!();