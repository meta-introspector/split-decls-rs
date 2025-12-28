macro_rules! Predefined {
    () => {
        struct Predefined { idx : u32 , span_of_name : Span , }
    };
}

Predefined!();