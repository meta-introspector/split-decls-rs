macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! same_scope {
    () => {
        deps!();
        pub (crate) fn same_scope (a : Cursor , b : Cursor) -> bool { ptr :: eq (a . scope , b . scope) }
    };
}

same_scope!()