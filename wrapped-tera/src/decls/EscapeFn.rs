macro_rules! EscapeFn {
    () => {
        # [doc = " The escape function type definition"] pub type EscapeFn = fn (& str) -> String ;
    };
}

EscapeFn!();