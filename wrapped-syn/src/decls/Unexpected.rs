macro_rules! Unexpected {
    () => {
        pub (crate) enum Unexpected { None , Some (Span , Delimiter) , Chain (Rc < Cell < Unexpected > >) , }
    };
}

Unexpected!()