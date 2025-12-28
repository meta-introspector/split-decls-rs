macro_rules! deps {
    () => {
        Span!();
        SyntaxContext!();
        HygieneData!();
    };
}

macro_rules! walk_chain {
    () => {
        deps!();
        pub fn walk_chain (span : Span , to : SyntaxContext) -> Span { HygieneData :: with (| data | data . walk_chain (span , to)) }
    };
}

walk_chain!()