macro_rules! deps {
    () => {
        Element!();
        Position!();
        SyntaxNode!();
    };
}

macro_rules! append_child_raw {
    () => {
        deps!();
        pub fn append_child_raw (node : & (impl Into < SyntaxNode > + Clone) , child : impl Element) { let position = Position :: last_child_of (node) ; insert_raw (position , child) ; }
    };
}

append_child_raw!();