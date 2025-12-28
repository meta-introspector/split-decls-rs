macro_rules! Term {
    () => {
        pub (crate) struct Term { spec : ColorSpec , stream : Stream , start_of_line : bool , }
    };
}

Term!()