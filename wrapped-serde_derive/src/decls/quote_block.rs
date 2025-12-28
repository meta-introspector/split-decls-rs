macro_rules! deps {
    () => {
        Fragment!();
    };
}

macro_rules! quote_block {
    () => {
        deps!();
        macro_rules ! quote_block { ($ ($ tt : tt) *) => { $ crate :: fragment :: Fragment :: Block (quote ! ($ ($ tt) *)) } }
    };
}

quote_block!()