macro_rules! deps {
    () => {
        Lookahead1!();
        Cursor!();
    };
}

macro_rules! new {
    () => {
        deps!();
        pub (crate) fn new (scope : Span , cursor : Cursor) -> Lookahead1 { Lookahead1 { scope , cursor , comparisons : RefCell :: new (Vec :: new ()) , } }
    };
}

new!()