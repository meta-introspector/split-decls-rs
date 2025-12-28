macro_rules! deps {
    () => {
        SyntaxNode!();
        Element!();
        Position!();
    };
}

macro_rules! append_child {
    () => {
        deps!();
        pub fn append_child (node : & (impl Into < SyntaxNode > + Clone) , child : impl Element) { let position = Position :: last_child_of (node) ; insert (position , child) ; }
    };
}

append_child!()