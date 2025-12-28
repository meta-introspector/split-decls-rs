macro_rules! SYMBOL_NAME {
    () => {
        const SYMBOL_NAME : Symbol = sym :: rustc_symbol_name ;
    };
}

SYMBOL_NAME!();