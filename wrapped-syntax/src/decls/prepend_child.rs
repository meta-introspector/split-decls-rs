macro_rules! deps {
    () => {
        Position!();
        SyntaxNode!();
        Element!();
    };
}

macro_rules! prepend_child {
    () => {
        deps!();
        pub fn prepend_child (node : & (impl Into < SyntaxNode > + Clone) , child : impl Element) { let position = Position :: first_child_of (node) ; insert (position , child) ; }
    };
}

prepend_child!()